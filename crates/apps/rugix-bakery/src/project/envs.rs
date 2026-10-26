//! Discover Dockerfile environments independently of recipes and layers.
//!
//! [`Envs`] resolves local and repository-qualified names to [`EnvDefinition`]s.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use reportify::{whatever, ResultExt};

use crate::config::envs::EnvInfo;
use crate::utils::idx_vec::IdxVec;
use crate::BakeryResult;

use super::repositories::{ProjectRepositories, RepositoryIdx};

/// An environment whose containing directory is its Docker build context.
#[derive(Debug)]
pub struct EnvDefinition {
    pub context: PathBuf,
}

impl EnvDefinition {
    /// Describe the definition without building it or opening the image store.
    pub fn info(&self, name: &str) -> EnvInfo {
        EnvInfo {
            name: name.into(),
            dockerfile: self
                .context
                .join("Dockerfile")
                .to_string_lossy()
                .into_owned()
                .into(),
            context: self.context.to_string_lossy().into_owned().into(),
        }
    }
}

/// Environment definitions indexed by their owning repository.
#[derive(Debug)]
pub struct Envs {
    repositories: Arc<ProjectRepositories>,
    tables: IdxVec<RepositoryIdx, BTreeMap<String, EnvDefinition>>,
}

impl Envs {
    /// Discover immediate `envs/<name>/Dockerfile` definitions in loaded repositories.
    #[tracing::instrument(skip_all, level = "debug")]
    pub fn load(repositories: Arc<ProjectRepositories>) -> BakeryResult<Self> {
        let mut tables = IdxVec::new();
        for (_, repository) in repositories.iter() {
            let mut table = BTreeMap::new();
            let directory = repository.source.dir.join("envs");
            if directory.is_dir() {
                for entry in fs::read_dir(&directory).whatever("unable to read envs directory")? {
                    let entry = entry.whatever("unable to read env directory entry")?;
                    let name = entry
                        .file_name()
                        .into_string()
                        .map_err(|_| whatever!("env name is not valid UTF-8"))?;
                    let context = entry.path();
                    if name.starts_with('.')
                        || !context.is_dir()
                        || !context.join("Dockerfile").is_file()
                    {
                        continue;
                    }
                    table.insert(
                        name,
                        EnvDefinition {
                            context: fs::canonicalize(context)
                                .whatever("unable to resolve env directory")?,
                        },
                    );
                }
            }
            tables.push(table);
        }
        Ok(Self {
            repositories,
            tables,
        })
    }

    /// Resolve a local name or a direct repository reference, such as `vendor/signing`.
    pub fn lookup(&self, repository: RepositoryIdx, name: &str) -> BakeryResult<&EnvDefinition> {
        let (repository, name) = match name.split_once('/') {
            Some(("core", name)) => (self.repositories.core_repository, name),
            Some((alias, name)) => (
                *self.repositories[repository]
                    .repositories
                    .get(alias)
                    .ok_or_else(|| whatever!("unable to find env repository {alias:?}"))?,
                name,
            ),
            None => (repository, name),
        };
        self.tables[repository].get(name).ok_or_else(|| {
            whatever!("unable to find env {name:?}; expected envs/<name>/Dockerfile")
        })
    }

    /// List references visible from the project, sorted by name.
    pub fn available(&self) -> BTreeMap<String, &EnvDefinition> {
        let root = self.repositories.root_repository;
        let mut result = BTreeMap::new();
        for (name, definition) in &self.tables[root] {
            result.insert(name.clone(), definition);
        }
        let mut dependencies = self.repositories[root].repositories.clone();
        dependencies.insert("core".into(), self.repositories.core_repository);
        for (alias, repository) in dependencies {
            for (name, definition) in &self.tables[repository] {
                result.insert(format!("{alias}/{name}"), definition);
            }
        }
        result
    }
}
