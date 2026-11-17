//! Execution order of the recipes applied by a layer or mixin.
//!
//! Recipes run in an order that satisfies their `before` and `after` constraints.
//! Among the recipes whose constraints are satisfied, higher priorities run first and
//! repository and recipe names break ties. Without constraints, recipes run in plain
//! priority order.

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::ops::Deref;

use reportify::{bail, ResultExt};

use crate::project::library::{Library, RecipeIdx};
use crate::project::recipes::Recipe;
use crate::BakeryResult;

/// Orders the given recipes for execution.
///
/// Constraints naming recipes outside of the given recipes have no effect. Fails if a
/// constraint names an unknown recipe or the constraints form a cycle.
pub(crate) fn order_recipes(
    library: &Library,
    recipes: impl IntoIterator<Item = RecipeIdx>,
) -> BakeryResult<Vec<RecipeIdx>> {
    let mut recipes = recipes.into_iter().collect::<Vec<_>>();
    recipes.sort_by(|left, right| compare_priority(library, *left, *right));
    let ranks = recipes
        .iter()
        .enumerate()
        .map(|(rank, recipe)| (*recipe, rank))
        .collect::<HashMap<_, _>>();
    let resolve = |recipe: &Recipe, name: &str| -> BakeryResult<Option<usize>> {
        let other = library
            .try_lookup(recipe.repository, name)
            .message(|| format!("while ordering recipe {}", recipe.path.display()))?;
        Ok(ranks.get(&other).copied())
    };
    let mut predecessors = vec![Vec::new(); recipes.len()];
    let mut successors = vec![Vec::new(); recipes.len()];
    for (rank, recipe) in recipes.iter().enumerate() {
        let recipe = &library.recipes[*recipe];
        for name in recipe.config.after.as_deref().unwrap_or_default() {
            if let Some(other) = resolve(recipe, name)? {
                predecessors[rank].push(other);
                successors[other].push(rank);
            }
        }
        for name in recipe.config.before.as_deref().unwrap_or_default() {
            if let Some(other) = resolve(recipe, name)? {
                predecessors[other].push(rank);
                successors[rank].push(other);
            }
        }
    }
    let mut pending = predecessors.iter().map(Vec::len).collect::<Vec<_>>();
    let mut ready = (0..recipes.len())
        .filter(|rank| pending[*rank] == 0)
        .collect::<BTreeSet<_>>();
    let mut order = Vec::with_capacity(recipes.len());
    while let Some(rank) = ready.pop_first() {
        order.push(recipes[rank]);
        for successor in &successors[rank] {
            pending[*successor] -= 1;
            if pending[*successor] == 0 {
                ready.insert(*successor);
            }
        }
    }
    if order.len() < recipes.len() {
        let cycle = find_cycle(&predecessors, &pending)
            .into_iter()
            .map(|rank| library.recipes[recipes[rank]].path.display().to_string())
            .collect::<Vec<_>>();
        bail!(
            "recipe ordering constraints form a cycle: {}",
            cycle.join(" -> ")
        );
    }
    Ok(order)
}

/// Compares recipes by descending priority, then by repository and recipe name.
fn compare_priority(library: &Library, left: RecipeIdx, right: RecipeIdx) -> Ordering {
    let left = &library.recipes[left];
    let right = &library.recipes[right];
    let repository_id =
        |recipe: &Recipe| library.repositories[recipe.repository].source.id.as_str();
    right
        .config
        .priority
        .unwrap_or_default()
        .cmp(&left.config.priority.unwrap_or_default())
        .then_with(|| repository_id(left).cmp(repository_id(right)))
        .then_with(|| left.name.deref().cmp(right.name.deref()))
}

/// Finds a cycle among the recipes that could not be ordered.
///
/// Every unordered recipe has an unordered predecessor, so following these
/// predecessors eventually revisits a recipe. Returns the cycle in execution order,
/// starting and ending with the same recipe.
fn find_cycle(predecessors: &[Vec<usize>], pending: &[usize]) -> Vec<usize> {
    let mut current = pending
        .iter()
        .position(|count| *count > 0)
        .expect("an incomplete order leaves some recipe unordered");
    let mut path = Vec::new();
    let mut positions = HashMap::new();
    loop {
        if let Some(start) = positions.insert(current, path.len()) {
            let mut cycle = path.split_off(start);
            cycle.push(current);
            cycle.reverse();
            return cycle;
        }
        path.push(current);
        current = *predecessors[current]
            .iter()
            .find(|predecessor| pending[**predecessor] > 0)
            .expect("an unordered recipe has an unordered predecessor");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::sync::Arc;

    use tempfile::{tempdir, TempDir};

    use crate::config::parse_config;
    use crate::config::repositories::{PathSourceConfig, SourceConfig};
    use crate::project::library::Library;
    use crate::project::repositories::{ProjectRepositories, Repository, RepositoryIdx, Source};
    use crate::utils::idx_vec::{Idx, IdxVec};

    use super::order_recipes;

    /// Constraints override priorities, and priorities order the recipes whose
    /// constraints are satisfied. Constraints on recipes outside the set are ignored.
    #[test]
    fn constraints_take_precedence_over_priorities() {
        let (_directory, library) = recipe_library(&[
            ("late", "priority = 100\nafter = [\"early\"]"),
            ("middle", "priority = 50\nafter = [\"outside\"]"),
            ("early", ""),
            ("first", "priority = -10\nbefore = [\"middle\"]"),
            ("outside", "priority = 200"),
        ]);
        assert_eq!(
            order(&library, &["late", "middle", "early", "first"]).unwrap(),
            ["early", "late", "first", "middle"]
        );
    }

    /// Cyclic constraints fail with the cycle, excluding recipes that are only blocked
    /// by it.
    #[test]
    fn cyclic_constraints_are_rejected() {
        let (_directory, library) = recipe_library(&[
            ("a", "before = [\"b\"]\nafter = [\"c\"]"),
            ("b", ""),
            ("c", "after = [\"b\"]"),
            ("blocked", "after = [\"a\"]"),
        ]);
        let error = order(&library, &["a", "b", "c", "blocked"]).unwrap_err();
        let root = library.repositories.root_repository;
        let path = |name: &&str| {
            let recipe = library.lookup(root, name).unwrap();
            library.recipes[recipe].path.display().to_string()
        };
        assert!(
            [
                ["a", "b", "c", "a"],
                ["b", "c", "a", "b"],
                ["c", "a", "b", "c"]
            ]
            .iter()
            .any(|cycle| error.contains(&cycle.iter().map(path).collect::<Vec<_>>().join(" -> "))),
            "{error}"
        );
        assert!(!error.contains("blocked"), "{error}");
    }

    /// Orders the named recipes and returns their names in execution order.
    fn order(library: &Library, names: &[&str]) -> Result<Vec<String>, String> {
        let root = library.repositories.root_repository;
        let recipes = names.iter().map(|name| library.lookup(root, name).unwrap());
        order_recipes(library, recipes)
            .map(|order| {
                order
                    .into_iter()
                    .map(|recipe| library.recipes[recipe].name.to_string())
                    .collect()
            })
            .map_err(|error| error.to_string())
    }

    /// Loads a single repository with the given recipe configurations.
    fn recipe_library(recipes: &[(&str, &str)]) -> (TempDir, Library) {
        let directory = tempdir().unwrap();
        for (name, config) in recipes {
            let recipe_dir = directory.path().join("recipes").join(name);
            fs::create_dir_all(&recipe_dir).unwrap();
            fs::write(recipe_dir.join("recipe.toml"), config).unwrap();
        }
        let source = Source::materialize(
            SourceConfig::Path(PathSourceConfig {
                path: directory.path().to_str().unwrap().into(),
            }),
            directory.path(),
            false,
        )
        .unwrap();
        let mut repositories = IdxVec::new();
        repositories.push(Repository {
            idx: repositories.next_idx(),
            source,
            config: parse_config("").unwrap(),
            repositories: HashMap::new(),
        });
        let root = RepositoryIdx::from_usize(0);
        let library = Library::load(Arc::new(ProjectRepositories {
            repositories,
            root_repository: root,
            core_repository: root,
        }))
        .unwrap();
        (directory, library)
    }
}
