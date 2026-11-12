use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use sha1::{Digest, Sha1};
use tracing::{debug, warn};

use xscript::{read_str, run, LocalEnv, Run};

use reportify::{bail, ResultExt};

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
        let patches = project
            .config()
            .patch
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect::<HashMap<_, _>>();
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
        for patch in repositories.unused_patches() {
            warn!(
                "patch for source {} does not match any repository",
                patch.key
            );
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
    /// Patches replacing the sources of repositories, keyed by url.
    patches: HashMap<String, SourcePatch>,
    /// URLs of patches that matched at least one repository source.
    used_patches: HashSet<String>,
    /// Path to the project's root directory.
    root_dir: PathBuf,
}

/// A patch replacing the source of a repository.
#[derive(Debug, Clone)]
struct SourcePatch {
    /// The patch key, as declared in the project configuration.
    key: String,
    /// The replacement source.
    config: SourceConfig,
}

impl RepositoriesLoader {
    /// Create an empty collection of repositories.
    pub fn new(root_dir: impl AsRef<Path>, patches: HashMap<String, SourceConfig>) -> Self {
        let root_dir = root_dir.as_ref();
        let patches = patches
            .into_iter()
            .map(|(key, config)| {
                let url = normalize_git_url(&key);
                (url.clone(), SourcePatch { key, config })
            })
            .collect();
        Self {
            repositories: IdxVec::new(),
            source_to_repository: HashMap::new(),
            patches,
            used_patches: HashSet::new(),
            root_dir: root_dir.to_path_buf(),
        }
    }

    /// The patches that did not match any repository source.
    fn unused_patches(&self) -> impl Iterator<Item = &SourcePatch> {
        self.patches
            .iter()
            .filter_map(|(url, patch)| (!self.used_patches.contains(url)).then_some(patch))
    }

    /// Resolve a source configuration against the declared patches.
    ///
    /// Patches declared in the project configuration map the Git URL of a
    /// repository (root or transitive) to a replacement source. The original
    /// `dir` of the declaration is preserved: a patched path source is
    /// extended with the original `dir`, and a patched Git source without its
    /// own `dir` inherits the original one. Sources that are not Git sources,
    /// or whose URL matches no patch, are returned unchanged. Patches are
    /// applied exactly once.
    fn resolve_patch(&mut self, config: SourceConfig) -> SourceConfig {
        let SourceConfig::Git(git_config) = &config else {
            return config;
        };
        let url = normalize_git_url(&git_config.url);
        let Some(patch) = self.patches.get(&url) else {
            return config;
        };
        self.used_patches.insert(url);
        let patch_config = patch.config.clone();
        let patched = match patch_config {
            SourceConfig::Path(mut path_config) => {
                if let Some(dir) = &git_config.dir {
                    let path = path_config.path.trim_end_matches('/');
                    path_config.path = format!("{path}/{dir}").into();
                }
                SourceConfig::Path(path_config)
            }
            SourceConfig::Git(mut replacement) => {
                if replacement.dir.is_none() {
                    replacement.dir = git_config.dir.clone();
                }
                SourceConfig::Git(replacement)
            }
        };
        debug!(
            "patching repository source {} (patch key {}) with {:?}",
            git_config.url, patch.key, patched
        );
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
        self.load_repository(
            Source::materialize(
                SourceConfig::Path(PathSourceConfig { path: "".into() }),
                &self.root_dir,
                update,
            )?,
            RepositoryConfig {
                name: Some("root".to_owned().into()),
                description: None,
                repositories: Some(
                    repositories
                        .into_iter()
                        .map(|(key, value)| (key.into(), value))
                        .collect::<::sidex::types::Map<_, _>>(),
                ),
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
        let config = self.resolve_patch(config);
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
                repositories.insert(name.clone(), self.load_source(source.clone(), update)?);
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

/// Normalize a Git URL for patch matching.
///
/// URLs are matched modulo trailing slashes and an optional `.git` suffix,
/// so that `https://github.com/x/repo`, `https://github.com/x/repo.git`, and
/// `https://github.com/x/repo/` all refer to the same repository.
fn normalize_git_url(url: &str) -> String {
    let url = url.trim_end_matches('/');
    let url = url.strip_suffix(".git").unwrap_or(url);
    url.to_owned()
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
        let patches = patches
            .iter()
            .map(|(key, config)| ((*key).to_owned(), config.clone()))
            .collect::<HashMap<_, _>>();
        RepositoriesLoader::new("", patches)
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

    #[test]
    fn patch_path_replacement_preserves_dir() {
        let mut loader = loader(&[("https://github.com/x/repo.git", path_source("../recipes"))]);
        let patched = loader.resolve_patch(git_source("https://github.com/x/repo", Some("unipi")));
        assert_eq!(
            patched,
            path_source("../recipes/unipi"),
            "the original `dir` must be appended to the patched path"
        );
    }

    #[test]
    fn patch_path_replacement_without_dir_stays() {
        let mut loader = loader(&[("https://github.com/x/repo", path_source("../recipes"))]);
        let patched = loader.resolve_patch(git_source("https://github.com/x/repo.git", None));
        assert_eq!(patched, path_source("../recipes"));
    }

    #[test]
    fn patch_git_replacement_inherits_dir() {
        let mut loader = loader(&[(
            "https://github.com/x/repo.git",
            git_source("https://gitlab.com/y/repo", None),
        )]);
        let patched = loader.resolve_patch(git_source("https://github.com/x/repo", Some("unipi")));
        assert_eq!(
            patched,
            git_source("https://gitlab.com/y/repo", Some("unipi"))
        );
    }

    #[test]
    fn patch_git_replacement_overrides_dir() {
        let mut loader = loader(&[(
            "https://github.com/x/repo",
            git_source("https://gitlab.com/y/repo", Some("other")),
        )]);
        let patched =
            loader.resolve_patch(git_source("https://github.com/x/repo.git", Some("unipi")));
        assert_eq!(
            patched,
            git_source("https://gitlab.com/y/repo", Some("other"))
        );
    }

    #[test]
    fn patch_matches_urls_modulo_suffix_and_slash() {
        let mut loader = loader(&[("https://github.com/x/repo", path_source("../recipes"))]);
        for url in [
            "https://github.com/x/repo",
            "https://github.com/x/repo.git",
            "https://github.com/x/repo/",
        ] {
            let patched = loader.resolve_patch(git_source(url, None));
            assert_eq!(patched, path_source("../recipes"), "url: {url}");
        }
    }

    #[test]
    fn path_sources_are_not_patched() {
        let mut loader = loader(&[("../other", path_source("vendor/other"))]);
        let config = path_source("../other");
        let patched = loader.resolve_patch(config.clone());
        assert_eq!(patched, config, "only Git sources may be patched");
    }

    #[test]
    fn unmatched_sources_are_returned_unchanged() {
        let mut loader = loader(&[("https://github.com/x/repo", path_source("../recipes"))]);
        let config = git_source("https://github.com/x/other", None);
        let patched = loader.resolve_patch(config.clone());
        assert_eq!(patched, config);
        assert_eq!(loader.unused_patches().count(), 1);
    }

    #[test]
    fn matched_patches_are_tracked() {
        let mut loader = loader(&[
            ("https://github.com/x/repo", path_source("../recipes")),
            ("https://github.com/x/unused", path_source("../unused")),
        ]);
        loader.resolve_patch(git_source("https://github.com/x/repo.git", None));
        let unused = loader
            .unused_patches()
            .map(|patch| patch.key.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            unused,
            vec!["https://github.com/x/unused".to_owned()],
            "only URLs that matched a source must count as used"
        );
    }

    #[test]
    fn normalize_git_url_strips_suffix_and_slashes() {
        assert_eq!(
            normalize_git_url("https://github.com/x/repo.git/"),
            "https://github.com/x/repo"
        );
        assert_eq!(
            normalize_git_url("https://gitlab.com/y/repo//"),
            "https://gitlab.com/y/repo"
        );
        assert_eq!(normalize_git_url("/local/repo"), "/local/repo");
    }
}
