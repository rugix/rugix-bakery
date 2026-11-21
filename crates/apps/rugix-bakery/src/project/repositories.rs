use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use sha1::{Digest, Sha1};
use tracing::{debug, warn};

use xscript::{read_str, run, LocalEnv, Run};

use reportify::{bail, ResultExt};
use sidex::types::{Map, Str};

use crate::config::repositories::{
    GitSourceConfig, PathSourceConfig, RepositoryConfig, SourceConfig,
};
use crate::utils::idx_vec::{new_idx_type, IdxVec};
use crate::{paths, BakeryResult};

use super::ProjectRef;

#[derive(Debug)]
#[non_exhaustive]
pub struct ProjectRepositories {
    pub repositories: IdxVec<RepositoryIdx, Repository>,
    pub root_repository: RepositoryIdx,
    pub core_repository: RepositoryIdx,
}

impl ProjectRepositories {
    pub fn load(project: &ProjectRef) -> BakeryResult<Self> {
        let patches = project.config().patch.clone().unwrap_or_default();
        let mut repositories = RepositoriesLoader::new(project.dir(), patches);
        let core = repositories.load_source(
            SourceConfig::Path(PathSourceConfig {
                path: (paths::repositories_dir()
                    .join("core")
                    .to_string_lossy()
                    .into_owned())
                .into(),
            }),
            false,
        )?;
        let root = repositories.load_root(
            project
                .config()
                .repositories
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect::<::std::collections::HashMap<_, _>>(),
            true,
        )?;
        for alias in repositories.unused_patches() {
            warn!("patch for repository {alias} does not match any repository declaration");
        }
        Ok(Self {
            repositories: repositories.repositories.map(|_, repo| repo.unwrap()),
            root_repository: root,
            core_repository: core,
        })
    }

    /// Iterator over the loaded repositories.
    pub fn iter(&self) -> impl Iterator<Item = (RepositoryIdx, &Repository)> {
        self.repositories.iter()
    }
}

impl std::ops::Index<RepositoryIdx> for ProjectRepositories {
    type Output = Repository;

    fn index(&self, index: RepositoryIdx) -> &Self::Output {
        &self.repositories[index]
    }
}

/// A collection of repositories.
#[derive(Debug)]
struct RepositoriesLoader {
    /// The repositories of the collection.
    repositories: IdxVec<RepositoryIdx, Option<Repository>>,
    /// Table for finding repositories by their source.
    source_to_repository: HashMap<SourceId, RepositoryIdx>,
    /// Patches replacing the sources of repositories, keyed by repository alias.
    patches: Map<Str, SourceConfig>,
    /// Aliases of patches that matched at least one repository declaration.
    used_patches: HashSet<Str>,
    /// Path to the project's root directory.
    root_dir: PathBuf,
}

impl RepositoriesLoader {
    /// Create an empty collection of repositories.
    pub fn new(root_dir: impl AsRef<Path>, patches: Map<Str, SourceConfig>) -> Self {
        let root_dir = root_dir.as_ref();
        Self {
            repositories: IdxVec::new(),
            source_to_repository: HashMap::new(),
            patches,
            used_patches: HashSet::new(),
            root_dir: root_dir.to_path_buf(),
        }
    }

    /// The aliases of patches that did not match any repository declaration.
    ///
    /// The aliases are yielded in the order in which the patches were
    /// declared in the configuration files (the sidex `Map` preserves
    /// insertion order).
    fn unused_patches(&self) -> impl Iterator<Item = &Str> {
        self.patches
            .keys()
            .filter(|alias| !self.used_patches.contains(*alias))
    }

    /// Patch the source of the repository declared under the given alias.
    fn patch_source(&mut self, alias: &str, source: SourceConfig) -> SourceConfig {
        let Some(patched) = self.patches.get(alias).cloned() else {
            return source;
        };
        self.used_patches.insert(alias.into());
        debug!("patching repository {alias} with {:?}", patched);
        patched
    }

    /// Load the repository from the project's root directory.
    ///
    /// The *update* flag indicates whether remote repositories should be updated.
    pub fn load_root(
        &mut self,
        repositories: HashMap<String, SourceConfig>,
        update: bool,
    ) -> BakeryResult<RepositoryIdx> {
        let repositories = repositories
            .into_iter()
            .map(|(alias, source)| (alias.into(), source))
            .collect::<Map<_, _>>();
        self.load_repository(
            Source::materialize(
                SourceConfig::Path(PathSourceConfig { path: "".into() }),
                &self.root_dir,
                update,
            )?,
            RepositoryConfig {
                name: Some("root".to_owned().into()),
                description: None,
                repositories: Some(repositories),
            },
            update,
        )
    }

    /// Load a repository from the given source and return its id.
    ///
    /// The *update* flag indicates whether remote repositories should be updated.
    pub fn load_source(
        &mut self,
        config: SourceConfig,
        update: bool,
    ) -> BakeryResult<RepositoryIdx> {
        let source_id = compute_source_id(&config);
        if let Some(id) = self.source_to_repository.get(&source_id).cloned() {
            let Some(repository) = &self.repositories[id] else {
                bail!("cycle while loading repository from:\n{:?}", config);
            };
            if repository.source.config == config {
                Ok(id)
            } else {
                bail!(
                    "incompatible repository sources:\n{:?}\n{:?}",
                    repository.source.config,
                    config,
                );
            }
        } else {
            let source = Source::materialize(config.clone(), &self.root_dir, update)?;
            let config_path = source.dir.join("rugix-repository.toml");
            let config =
                toml::from_str(&std::fs::read_to_string(&config_path).whatever_with(|_| {
                    format!("reading repository configuration from {config_path:?}")
                })?)
                .whatever_with(|_| {
                    format!("unable to parse repository configuration file {config_path:?}")
                })?;
            self.load_repository(source, config, update)
        }
    }

    /// Load a repository from an already materialized source and given config.
    fn load_repository(
        &mut self,
        source: Source,
        config: RepositoryConfig,
        update: bool,
    ) -> BakeryResult<RepositoryIdx> {
        if self.source_to_repository.contains_key(&source.id) {
            bail!("repository from {} has already been loaded", source.id);
        }
        debug!("loading repository from source {}", source.id);
        let idx = RepositoryIdx(self.repositories.len());
        self.repositories.push(None);
        self.source_to_repository.insert(source.id.clone(), idx);
        let mut repositories = HashMap::new();
        if let Some(dependencies) = &config.repositories {
            for (name, source) in dependencies {
                let source = self.patch_source(name, source.clone());
                repositories.insert(name.clone(), self.load_source(source, update)?);
            }
        }
        let repository = Repository {
            idx,
            source,
            config,
            repositories: repositories
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect::<::std::collections::HashMap<_, _>>(),
        };
        self.repositories[idx] = Some(repository);
        Ok(idx)
    }
}

new_idx_type! {
    /// An index uniquely identifying a repository in [`Repositories`].
    pub RepositoryIdx
}

/// A repository.
#[derive(Debug)]
pub struct Repository {
    /// The index of the repository.
    pub idx: RepositoryIdx,
    /// The source of the repository.
    pub source: Source,
    /// The configuration of the repository.
    pub config: RepositoryConfig,
    /// The repositories used by the repository.
    pub repositories: HashMap<String, RepositoryIdx>,
}

/// A source which has been materialized in a local directory.
#[derive(Debug, Clone)]
pub struct Source {
    /// The id of the source.
    pub id: SourceId,
    /// The definition of the source.
    pub config: SourceConfig,
    /// The directory where the source has been materialized.
    pub dir: PathBuf,
}

impl Source {
    /// Materialize the source within the given project root directory.
    ///
    /// The *update* flag indicates whether remote repositories should be updated.
    pub fn materialize(config: SourceConfig, root_dir: &Path, update: bool) -> BakeryResult<Self> {
        let id = compute_source_id(&config);
        debug!("materializing source {id}");
        let path = match &config {
            SourceConfig::Path(config) => root_dir.join(&config.path),
            SourceConfig::Git(config) => {
                let mut path = root_dir.join(".rugix/repositories");
                path.push(id.as_str());
                check_out_git_source(config, &path, update)?;
                if let Some(repository_path) = &config.dir {
                    path.push(repository_path);
                }
                path
            }
        };
        Ok(Self {
            id,
            config,
            dir: path,
        })
    }
}

/// Globally unique id of a source.
///
/// The id is computed by hashing the path or URL of a source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceId(Arc<str>);

impl SourceId {
    /// The string representation of the id.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The short string representation of the id.
    pub fn as_short_str(&self) -> &str {
        &self.as_str()[..6]
    }
}

impl Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Compute the globally unique id of the source.
fn compute_source_id(config: &SourceConfig) -> SourceId {
    let mut hasher = Sha1::new();
    match config {
        SourceConfig::Path(path_source) => {
            hasher.update(b"path");
            hasher.update(path_source.path.as_bytes());
        }
        SourceConfig::Git(git_source) => {
            hasher.update(b"git");
            hasher.update(git_source.url.as_bytes());
            if let Some(inner_path) = &git_source.dir {
                hasher.update(inner_path.as_bytes());
            }
        }
    }
    SourceId(hex::encode(&hasher.finalize()[..]).into())
}

/// Check out the Git repository in the given directory.
///
/// The *fetch* flag indicates whether updates should be fetched from the remote.
fn check_out_git_source(config: &GitSourceConfig, path: &Path, fetch: bool) -> BakeryResult<()> {
    if !path.exists() {
        run!(["git", "clone", &config.url, path]).whatever("unable to clone repository")?;
    }
    let env = LocalEnv::new(path);
    if fetch {
        run!(env, ["git", "fetch", "--all"]).whatever("unable to fetch updates of repository")?;
    }
    macro_rules! rev_parse {
        ($rev:literal) => {
            read_str!(env, ["git", "rev-parse", "--verify", $rev]).whatever("unable to parse rev")
        };
    }
    let mut commit = rev_parse!("refs/remotes/origin/HEAD^{{commit}}")?;
    if let Some(tag) = &config.tag {
        commit = rev_parse!("refs/tags/{tag}^{{commit}}")?;
    }
    if let Some(branch) = &config.branch {
        commit = rev_parse!("refs/remotes/origin/{branch}^{{commit}}")?;
    }
    if let Some(rev) = &config.rev {
        commit = rev_parse!("{rev}^{{commit}}")?;
    }
    let head = rev_parse!("HEAD^{{commit}}")?;
    if head != commit {
        run!(env, ["git", "checkout", commit]).whatever("error checking out commit")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loader(patches: &[(&str, SourceConfig)]) -> RepositoriesLoader {
        loader_at(Path::new(""), patches)
    }

    /// A loader rooted at the given directory with the given patches.
    fn loader_at(dir: &Path, patches: &[(&str, SourceConfig)]) -> RepositoriesLoader {
        let patches = patches
            .iter()
            .map(|(alias, config)| ((*alias).into(), config.clone()))
            .collect::<Map<Str, _>>();
        RepositoriesLoader::new(dir, patches)
    }

    fn git_source(url: &str, dir: Option<&str>) -> SourceConfig {
        SourceConfig::Git(GitSourceConfig {
            url: url.into(),
            tag: None,
            branch: None,
            rev: None,
            dir: dir.map(Into::into),
        })
    }

    fn path_source(path: &str) -> SourceConfig {
        SourceConfig::Path(PathSourceConfig { path: path.into() })
    }

    /// Create a repository directory with the given manifest content.
    fn write_repository(dir: &Path, manifest: &str) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join("rugix-repository.toml"), manifest).unwrap();
    }

    /// The aliases of unused patches, in the order in which the patches were
    /// declared (the sidex `Map` preserves insertion order).
    fn unused_aliases(loader: &RepositoriesLoader) -> Vec<String> {
        loader
            .unused_patches()
            .map(|alias| alias.to_string())
            .collect()
    }

    #[test]
    fn patched_declarations_are_replaced_verbatim() {
        let cases = [
            (
                "the original `dir` of a Git declaration is dropped",
                git_source("https://github.com/x/repo", Some("unipi")),
                path_source("../recipes"),
            ),
            (
                "path declarations are patched to Git sources alike",
                path_source("../dep"),
                git_source("https://github.com/x/repo", Some("unipi")),
            ),
        ];
        for (case, declaration, patch) in cases {
            let mut loader = loader(&[("dep", patch.clone())]);
            assert_eq!(loader.patch_source("dep", declaration), patch, "{case}");
            assert_eq!(unused_aliases(&loader), Vec::<String>::new(), "{case}");
        }
    }

    /// Aliases without a patch pass through unchanged; only aliases whose
    /// declaration was patched count as used.
    #[test]
    fn unmatched_aliases_pass_through_and_stay_unused() {
        let mut loader = loader(&[
            ("dep", path_source("../recipes")),
            ("other", path_source("../unused")),
        ]);
        let declaration = git_source("https://invalid.example/repo.git", None);
        assert_eq!(
            loader.patch_source("unknown", declaration.clone()),
            declaration,
            "declarations of aliases without a patch pass through unchanged"
        );
        assert_eq!(
            unused_aliases(&loader),
            ["dep", "other"],
            "no patch may count as used before one of its aliases is declared"
        );
        assert_eq!(
            loader.patch_source("dep", git_source("https://invalid.example/repo.git", None)),
            path_source("../recipes")
        );
        assert_eq!(
            unused_aliases(&loader),
            ["other"],
            "only aliases whose declaration was patched count as used"
        );
    }

    /// The root table's git repository is patched with a path source instead.
    #[test]
    fn root_table_is_patched() {
        let dir = tempfile::tempdir().unwrap();
        write_repository(&dir.path().join("dep"), r#"name = "dep""#);
        let mut loader = loader_at(dir.path(), &[("dep", path_source("dep"))]);
        let root = loader
            .load_root(
                [(
                    "dep".to_owned(),
                    git_source("https://invalid.example/repo.git", None),
                )]
                .into_iter()
                .collect(),
                false,
            )
            .unwrap();
        let dep = &loader.repositories[root].as_ref().unwrap().repositories["dep"];
        assert_eq!(
            loader.repositories[*dep].as_ref().unwrap().source.dir,
            dir.path().join("dep"),
            "the dependency must have been loaded from the patched source"
        );
        assert_eq!(unused_aliases(&loader), Vec::<String>::new());
    }

    /// Transitive tables are patched: a dependency declared by a loaded
    /// repository is redirected by the project-level patch.
    #[test]
    fn transitive_table_is_patched() {
        let dir = tempfile::tempdir().unwrap();
        write_repository(
            &dir.path().join("mid"),
            r#"
            name = "mid"
            [repositories]
            dep = { path = "../nowhere" }
            "#,
        );
        write_repository(&dir.path().join("dep"), r#"name = "dep""#);
        let mut loader = loader_at(dir.path(), &[("dep", path_source("dep"))]);
        let root = loader
            .load_root(
                [("mid".to_owned(), path_source("mid"))]
                    .into_iter()
                    .collect(),
                false,
            )
            .unwrap();
        let mid_idx = loader.repositories[root].as_ref().unwrap().repositories["mid"].clone();
        let mid = loader.repositories[mid_idx].as_ref().unwrap();
        assert!(
            mid.repositories.contains_key("dep"),
            "the patched transitive dependency must have been loaded"
        );
        assert_eq!(unused_aliases(&loader), Vec::<String>::new());
    }

    /// Two scopes patched to the same source collapse into one repository.
    #[test]
    fn two_scopes_patched_to_the_same_source_merge() {
        let dir = tempfile::tempdir().unwrap();
        write_repository(&dir.path().join("dep"), r#"name = "dep""#);
        // A second scope declaring `dep` differently; without the patch, it
        // would fail to load `../nowhere`.
        write_repository(
            &dir.path().join("mid"),
            r#"
            name = "mid"
            [repositories]
            dep = { path = "../nowhere" }
            "#,
        );
        let mut loader = loader_at(dir.path(), &[("dep", path_source("dep"))]);
        let root = loader
            .load_root(
                [
                    (
                        "dep".to_owned(),
                        git_source("https://invalid.example/repo.git", None),
                    ),
                    ("mid".to_owned(), path_source("mid")),
                ]
                .into_iter()
                .collect(),
                false,
            )
            .unwrap();
        let repository = loader.repositories[root].as_ref().unwrap();
        let root_dep = repository.repositories["dep"].clone();
        let mid_idx = repository.repositories["mid"].clone();
        let mid_dep = loader.repositories[mid_idx].as_ref().unwrap().repositories["dep"].clone();
        assert_eq!(
            root_dep, mid_dep,
            "all scopes patched to the same source must resolve to one repository"
        );
        assert_eq!(unused_aliases(&loader), Vec::<String>::new());
    }
}
