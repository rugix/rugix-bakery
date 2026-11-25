use std::collections::HashMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use sha1::{Digest, Sha1};
use tracing::debug;

use reportify::{bail, ResultExt};

use crate::config::repositories::{PathSourceConfig, RepositoryConfig, SourceConfig};
use crate::project::git::{check_out_git_source, compute_git_source_id, SourceId};
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
        let mut repositories = RepositoriesLoader::new(project.dir());
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
    /// Path to the project's root directory.
    root_dir: PathBuf,
}

impl RepositoriesLoader {
    /// Create an empty collection of repositories.
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        let root_dir = root_dir.as_ref();
        Self {
            repositories: IdxVec::new(),
            source_to_repository: HashMap::new(),
            root_dir: root_dir.to_path_buf(),
        }
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

/// Compute the globally unique id of the source.
fn compute_source_id(config: &SourceConfig) -> SourceId {
    match config {
        SourceConfig::Path(path_source) => {
            let mut hasher = Sha1::new();
            hasher.update(b"path");
            hasher.update(path_source.path.as_bytes());
            SourceId::from_hash(&hex::encode(&hasher.finalize()[..]))
        }
        SourceConfig::Git(git_source) => compute_git_source_id(git_source),
    }
}
