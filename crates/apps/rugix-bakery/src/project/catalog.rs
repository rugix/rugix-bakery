//! Discover and inspect rules, recipes, and layers without loading their build
//! dependencies.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use reportify::{bail, whatever, ErrorExt, ResultExt};

use crate::config::catalog::{
    DefinitionKind, LayerDefinition, LayerInfo, LayerVariant, RecipeInfo, RuleInfo,
};
use crate::config::systems::Architecture;
use crate::BakeryResult;

use super::repositories::{ProjectRepositories, RepositoryIdx};

/// Repository definitions visible from a project and its direct dependencies.
pub struct Catalog {
    repositories: Arc<ProjectRepositories>,
}

impl Catalog {
    pub fn new(repositories: Arc<ProjectRepositories>) -> Self {
        Self { repositories }
    }

    /// List names without parsing configurations or loading build dependencies.
    pub fn available(
        &self,
        kind: DefinitionKind,
        repository: Option<&str>,
    ) -> BakeryResult<BTreeSet<String>> {
        let root = self.repositories.root_repository;
        let mut owners = BTreeMap::from([(".".to_owned(), root)]);
        owners.extend(self.repositories[root].repositories.clone());
        owners.insert("core".to_owned(), self.repositories.core_repository);
        if let Some(repository) = repository {
            if !owners.contains_key(repository) {
                bail!("unknown repository alias {repository:?}");
            }
            owners.retain(|alias, _| alias == repository);
        }
        let mut available = BTreeSet::new();
        for (alias, owner) in owners {
            let directory = self.repositories[owner].source.dir.join(kind.directory());
            if !directory.is_dir() {
                continue;
            }
            for entry in fs::read_dir(directory).whatever("unable to read definitions directory")? {
                let entry = entry.whatever("unable to read definition directory entry")?;
                let path = entry.path();
                let filename = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| whatever!("definition name is not valid UTF-8"))?;
                if filename.starts_with('.') {
                    continue;
                }
                let name = if kind == DefinitionKind::Layer {
                    if !path.is_file()
                        || path.extension().is_none_or(|extension| extension != "toml")
                    {
                        continue;
                    }
                    let stem = path
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .ok_or_else(|| whatever!("invalid layer filename"))?;
                    layer_name(stem)?.0.to_owned()
                } else {
                    if !path.is_dir() || !path.join(kind.filename()).is_file() {
                        continue;
                    }
                    filename
                };
                validate_name(&name)?;
                let reference = if alias == "." {
                    name
                } else {
                    format!("{alias}/{name}")
                };
                available.insert(reference);
            }
        }
        Ok(available)
    }

    pub fn rule(&self, reference: &str) -> BakeryResult<RuleInfo> {
        let directory = self.directory(DefinitionKind::Rule, reference)?;
        Ok(RuleInfo {
            name: reference.into(),
            definition: crate::config::load_config(&directory.join("rule.toml"))?,
            documentation: documentation(&directory.join("README.md"))?,
        })
    }

    pub fn recipe(&self, reference: &str) -> BakeryResult<RecipeInfo> {
        let directory = self.directory(DefinitionKind::Recipe, reference)?;
        Ok(RecipeInfo {
            name: reference.into(),
            definition: crate::config::load_config(&directory.join("recipe.toml"))?,
            documentation: documentation(&directory.join("README.md"))?,
        })
    }

    /// Preserve the default and all architecture overrides as separate complete
    /// configurations.
    pub fn layer(&self, reference: &str) -> BakeryResult<LayerInfo> {
        let base = self.directory(DefinitionKind::Layer, reference)?;
        let name = base
            .file_name()
            .and_then(|name| name.to_str())
            .expect("validated definition name");
        let directory = base.parent().expect("layer directory has a parent");
        let mut variants = BTreeMap::new();
        for entry in fs::read_dir(directory).whatever("unable to read layers directory")? {
            let entry = entry.whatever("unable to read layer entry")?;
            let path = entry.path();
            if !path.is_file() || path.extension().is_none_or(|extension| extension != "toml") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            if stem != name && !stem.starts_with(&format!("{name}.")) {
                continue;
            }
            let (_, architecture) = layer_name(stem)?;
            variants.insert(
                architecture
                    .map(|architecture| architecture.as_str())
                    .unwrap_or(""),
                LayerVariant {
                    architecture,
                    config: crate::config::load_config(&path)?,
                },
            );
        }
        if variants.is_empty() {
            bail!("unable to find layer {reference:?}");
        }
        Ok(LayerInfo {
            name: reference.into(),
            definition: LayerDefinition {
                variants: variants.into_values().collect(),
            },
            documentation: documentation(&base.with_extension("md"))?,
        })
    }

    fn directory(&self, kind: DefinitionKind, reference: &str) -> BakeryResult<PathBuf> {
        let (repository, name) = resolve(
            &self.repositories,
            self.repositories.root_repository,
            reference,
        )?;
        Ok(self.repositories[repository]
            .source
            .dir
            .join(kind.directory())
            .join(name))
    }
}

impl DefinitionKind {
    pub fn directory(self) -> &'static str {
        match self {
            Self::Rule => "rules",
            Self::Recipe => "recipes",
            Self::Layer => "layers",
        }
    }

    fn filename(self) -> &'static str {
        match self {
            Self::Rule => "rule.toml",
            Self::Recipe => "recipe.toml",
            Self::Layer => "layer.toml",
        }
    }
}

/// Resolve names from the defining repository, including dependency aliases.
pub fn resolve<'a>(
    repositories: &ProjectRepositories,
    owner: RepositoryIdx,
    reference: &'a str,
) -> BakeryResult<(RepositoryIdx, &'a str)> {
    let (repository, name) = match reference.split_once('/') {
        Some(("core", name)) => (repositories.core_repository, name),
        Some((alias, name)) => (
            *repositories[owner]
                .repositories
                .get(alias)
                .ok_or_else(|| whatever!("unknown repository alias {alias:?}"))?,
            name,
        ),
        None => (owner, reference),
    };
    validate_name(name)?;
    Ok((repository, name))
}

/// Restrict build identifiers to one unambiguous filesystem component.
pub fn validate_name(name: &str) -> BakeryResult<()> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_.-".contains(&byte))
    {
        bail!("invalid build name {name:?}");
    }
    Ok(())
}

/// Missing reference documentation is valid; read errors remain visible.
fn documentation(path: &Path) -> BakeryResult<Option<sidex::types::Str>> {
    match fs::read_to_string(path) {
        Ok(documentation) => Ok(Some(documentation.into())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.whatever("unable to read reference documentation")),
    }
}

/// Decode a layer's optional architecture suffix using the same convention as the
/// builder.
fn layer_name(stem: &str) -> BakeryResult<(&str, Option<Architecture>)> {
    match stem.split_once('.') {
        Some((name, architecture)) => Ok((
            name,
            Some(
                architecture
                    .parse::<Architecture>()
                    .whatever("invalid layer architecture")?,
            ),
        )),
        None => Ok((stem, None)),
    }
}
