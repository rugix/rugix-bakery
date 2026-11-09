//! In-memory project representation.

use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use library::Library;
use reportify::ResultExt;
use repositories::ProjectRepositories;
use si_crypto_hashes::HashAlgorithm;
use tracing::{info, warn};

use crate::config::projects::ProjectConfig;
use crate::config::{load_config, LocalProjectOverrides};
use crate::BakeryResult;

pub mod catalog;
pub mod envs;
pub mod layers;
pub mod library;
pub mod mixins;
pub mod recipes;
pub mod repositories;

/// Shared reference to an in-memory project.
#[derive(Debug, Clone)]
pub struct ProjectRef {
    /// Shared project state.
    shared: Arc<ProjectShared>,
}

impl ProjectRef {
    /// Local project id.
    pub fn local_id(&self) -> &LocalProjectId {
        &self.shared.id
    }

    /// Project directory.
    pub fn dir(&self) -> &Path {
        &self.shared.dir
    }

    /// Project configuration.
    pub fn config(&self) -> &ProjectConfig {
        &self.shared.config
    }

    /// Retrieve the repositories of the project.
    ///
    /// This may load the repositories lazily.
    pub fn repositories(&self) -> BakeryResult<Arc<ProjectRepositories>> {
        let mut repositories = self.shared.lazy.repositories.lock().unwrap();
        if let Some(repositories) = &*repositories {
            Ok(repositories.clone())
        } else {
            *repositories = Some(ProjectRepositories::load(self).map(Arc::new)?);
            Ok(repositories.clone().unwrap())
        }
    }

    /// Retrieve the library of the project.
    ///
    /// This may load the library lazily.
    pub fn library(&self) -> BakeryResult<Arc<Library>> {
        let mut library = self.shared.lazy.library.lock().unwrap();
        if let Some(library) = &*library {
            Ok(library.clone())
        } else {
            *library = Some(Library::load(self.repositories()?).map(Arc::new)?);
            Ok(library.clone().unwrap())
        }
    }
}

/// Shared project state.
#[derive(Debug)]
struct ProjectShared {
    /// Project directory.
    dir: PathBuf,
    /// Local project id.
    id: LocalProjectId,
    /// Project configuration.
    config: Arc<ProjectConfig>,
    /// Lazily-loaded project data.
    lazy: ProjectLazy,
}

#[derive(Debug, Default)]
struct ProjectLazy {
    repositories: Mutex<Option<Arc<ProjectRepositories>>>,
    library: Mutex<Option<Arc<Library>>>,
}

/// Locally unique id of the project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProjectId(Arc<str>);

impl LocalProjectId {
    /// String representation of the project id.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Project loader.
#[derive(Debug)]
pub struct ProjectLoader {
    /// The project directory.
    project_dir: PathBuf,
    /// Path to the configuration file.
    config_file: Option<PathBuf>,
    /// Local project id.
    local_id: LocalProjectId,
}

impl ProjectLoader {
    /// Construct a new project loader with the given project directory.
    pub fn new(project_dir: &Path) -> Self {
        Self {
            project_dir: project_dir.to_path_buf(),
            config_file: None,
            local_id: local_id_from_bytes(project_dir.as_os_str().as_bytes()),
        }
    }

    /// Construct a new project loader from the current working directory.
    pub fn current_dir() -> BakeryResult<Self> {
        Ok(Self::new(
            &std::env::current_dir().whatever("unable to determine current directory")?,
        ))
    }

    /// Set the configuration file path relative to the project directory.
    pub fn with_config_file(mut self, config_file: Option<&Path>) -> Self {
        self.config_file = config_file.map(Path::to_path_buf);
        self
    }

    pub fn with_local_id(mut self, identity: &[u8]) -> Self {
        self.local_id = local_id_from_bytes(identity);
        self
    }

    /// The full path to the configuration file.
    fn config_path(&self) -> PathBuf {
        self.project_dir.join(
            self.config_file
                .as_deref()
                .unwrap_or_else(|| Path::new("rugix-bakery.toml")),
        )
    }

    /// The full path to the local overrides file.
    ///
    /// The overrides file is discovered next to the configuration file by
    /// replacing its extension:
    /// * `rugix-bakery.toml` -> `rugix-bakery.local.toml`,
    /// * `--config custom.toml` -> `custom.local.toml`.
    fn local_overrides_path(&self) -> PathBuf {
        self.config_path().with_extension("local.toml")
    }

    /// Load the project.
    pub fn load(self) -> BakeryResult<ProjectRef> {
        let mut config: ProjectConfig = load_config(&self.config_path())?;
        self.apply_local_overrides(&mut config)?;
        Ok(ProjectRef {
            shared: Arc::new(ProjectShared {
                dir: self.project_dir,
                config: config.into(),
                id: self.local_id,
                lazy: ProjectLazy::default(),
            }),
        })
    }

    /// Apply local overrides from a `*.local.toml` file next to the
    /// configuration file, if present.
    fn apply_local_overrides(&self, config: &mut ProjectConfig) -> BakeryResult<()> {
        let overrides_path = self.local_overrides_path();
        if !overrides_path.is_file() {
            return Ok(());
        }
        info!("applying local overrides from {}", overrides_path.display());
        let overrides: LocalProjectOverrides = load_config(&overrides_path)?;
        if let Some(local_patches) = overrides.patch {
            let patches = config.patch.get_or_insert_with(Default::default);
            for (url, source) in local_patches {
                if patches.contains_key(&url) {
                    warn!(
                        "local patch for {url} overrides the [patch] entry of {}",
                        self.config_path().display()
                    );
                }
                patches.insert(url, source);
            }
        }
        Ok(())
    }
}

fn local_id_from_bytes(bytes: &[u8]) -> LocalProjectId {
    LocalProjectId(
        HashAlgorithm::Sha256
            .hash::<Arc<[u8]>>(bytes)
            .raw_hex_string()[..10]
            .into(),
    )
}

#[cfg(test)]
mod project_tests {
    use super::*;
    use crate::config::repositories::SourceConfig;

    /// Local patch entries are merged over the shared ones and win on collision.
    #[test]
    fn local_overrides_merge_and_win() {
        let project_dir = tempfile::tempdir().unwrap();
        std::fs::write(
            project_dir.path().join("rugix-bakery.toml"),
            r#"
[patch]
"https://github.com/x/shared-only.git" = { path = "shared-only" }
"https://github.com/x/shadowed.git" = { path = "shared" }
"#,
        )
        .unwrap();
        std::fs::write(
            project_dir.path().join("rugix-bakery.local.toml"),
            r#"
[patch]
"https://github.com/x/shadowed.git" = { path = "local-wins" }
"https://github.com/x/local-only.git" = { path = "local-only" }
"#,
        )
        .unwrap();
        let project = ProjectLoader::new(project_dir.path()).load().unwrap();
        let patches = project.config().patch.as_ref().unwrap();
        assert_eq!(
            patches.get("https://github.com/x/shared-only.git").unwrap(),
            &SourceConfig::Path(crate::config::repositories::PathSourceConfig {
                path: "shared-only".into(),
            })
        );
        assert_eq!(
            patches.get("https://github.com/x/shadowed.git").unwrap(),
            &SourceConfig::Path(crate::config::repositories::PathSourceConfig {
                path: "local-wins".into(),
            }),
            "local patches must take precedence over shared ones"
        );
        assert_eq!(
            patches.get("https://github.com/x/local-only.git").unwrap(),
            &SourceConfig::Path(crate::config::repositories::PathSourceConfig {
                path: "local-only".into(),
            })
        );
    }

    /// Projects without local overrides load as before.
    #[test]
    fn local_overrides_are_optional() {
        let project_dir = tempfile::tempdir().unwrap();
        std::fs::write(
            project_dir.path().join("rugix-bakery.toml"),
            r#"
[patch]
"https://github.com/x/shared-only.git" = { path = "shared-only" }
"#,
        )
        .unwrap();
        let project = ProjectLoader::new(project_dir.path()).load().unwrap();
        let patches = project.config().patch.as_ref().unwrap();
        assert_eq!(patches.len(), 1);
    }

    /// The overrides path pairs with the configuration file, custom names included.
    #[test]
    fn local_overrides_path_follows_config_file() {
        let project_dir = tempfile::tempdir().unwrap();
        let loader = ProjectLoader::new(project_dir.path());
        assert_eq!(
            loader.local_overrides_path(),
            project_dir.path().join("rugix-bakery.local.toml")
        );
        let loader = loader.with_config_file(Some(Path::new("cfg/custom.toml")));
        assert_eq!(
            loader.local_overrides_path(),
            project_dir.path().join("cfg/custom.local.toml")
        );
    }
}
