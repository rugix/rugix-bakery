//! Shared cache lookup, output validation, and atomic publication for executable nodes.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use reportify::{bail, ResultExt};
use sidex::types::{Map, Str};
use tempfile::TempDir;
use tracing::info;

use crate::config::rules::{ArtifactKind, RuleOutput};
use crate::BakeryResult;

use super::dependencies::ProjectDependencies;

/// Completed node outputs and uncached workspaces retained for this build session.
pub struct Results {
    root: PathBuf,
    project: PathBuf,
    temporary: Vec<TempDir>,
}

/// Private execution paths shared by command and layer implementations.
pub struct Workspace {
    directory: TempDir,
    pub outputs: PathBuf,
    pub registrations: PathBuf,
}

impl Results {
    pub fn new(root: &Path, project: &Path) -> BakeryResult<Self> {
        fs::create_dir_all(root).whatever("unable to create result cache")?;
        Ok(Self {
            root: fs::canonicalize(root).whatever("unable to resolve result cache")?,
            project: fs::canonicalize(project).whatever("unable to resolve project directory")?,
            temporary: Vec::new(),
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Preserve an uncached standalone output after the session ends.
    pub fn keep_artifact(&mut self, artifact: &Path) {
        if let Some(index) = self
            .temporary
            .iter()
            .position(|directory| artifact.starts_with(directory.path()))
        {
            let _ = self.temporary.swap_remove(index).keep();
        }
    }

    /// Reuse or execute a node, publishing only validated, complete results.
    pub fn execute(
        &mut self,
        label: &str,
        key: Option<&str>,
        outputs: &Map<Str, RuleOutput>,
        build: impl FnOnce(&Workspace) -> BakeryResult<()>,
    ) -> BakeryResult<BTreeMap<Str, PathBuf>> {
        let dependency_record =
            key.map(|key| self.root.join("dependencies").join(format!("{key}.json")));
        if let Some((key, record)) = key.zip(dependency_record.as_ref()) {
            if let Some(dependencies) = ProjectDependencies::load(record)? {
                if dependencies.is_current(&self.project)? {
                    let destination = self.root.join(dependencies.cache_key(key)?);
                    if destination.is_dir() {
                        info!("Reusing {label}.");
                        return validate_outputs(label, outputs, &destination);
                    }
                }
            }
        }

        info!("Building {label}.");
        let directory = tempfile::Builder::new()
            .prefix("task-")
            .tempdir_in(&self.root)
            .whatever("unable to create execution workspace")?;
        let workspace = Workspace {
            outputs: directory.path().join("outputs"),
            registrations: directory.path().join("rebuild-if-changed.txt"),
            directory,
        };
        fs::create_dir(&workspace.outputs).whatever("unable to create output directory")?;
        fs::write(&workspace.registrations, "")
            .whatever("unable to create dependency registrations")?;
        let result = (|| {
            build(&workspace)?;
            let artifacts = validate_outputs(label, outputs, &workspace.outputs)?;
            let dependencies =
                ProjectDependencies::capture(&self.project, &workspace.registrations)?;
            Ok((artifacts, dependencies))
        })();
        let (artifacts, dependencies) = match result {
            Ok(result) => result,
            Err(error) => {
                // Retain diagnostic files referenced by a failed layer build.
                if workspace.outputs.join("build.log").is_file() {
                    let _ = workspace.directory.keep();
                }
                return Err(error);
            }
        };

        if let Some((key, record)) = key.zip(dependency_record.as_ref()) {
            let destination = self.root.join(dependencies.cache_key(key)?);
            if !destination.is_dir() {
                fs::rename(&workspace.outputs, &destination)
                    .whatever("unable to publish cached outputs")?;
            }
            let artifacts = validate_outputs(label, outputs, &destination)?;
            dependencies.save(record)?;
            Ok(artifacts)
        } else {
            self.temporary.push(workspace.directory);
            Ok(artifacts)
        }
    }
}

impl Workspace {
    pub fn path(&self) -> &Path {
        self.directory.path()
    }
}

/// Reject missing outputs, incompatible kinds, and paths escaping the output directory.
fn validate_outputs(
    label: &str,
    definitions: &Map<Str, RuleOutput>,
    root: &Path,
) -> BakeryResult<BTreeMap<Str, PathBuf>> {
    let root = fs::canonicalize(root).whatever("unable to resolve outputs")?;
    let mut outputs = BTreeMap::new();
    for (name, output) in definitions {
        let path = fs::canonicalize(root.join(&output.path))
            .whatever_with(|_| format!("{label} did not produce output {name:?}"))?;
        if !path.starts_with(&root) {
            bail!("output {name:?} escapes its output directory");
        }
        let metadata = fs::metadata(&path).whatever("unable to inspect output")?;
        let matches = match output.kind {
            ArtifactKind::File => metadata.is_file(),
            ArtifactKind::Directory => metadata.is_dir(),
        };
        if !matches {
            bail!("output {name:?} has an incompatible artifact kind");
        }
        outputs.insert(name.clone(), path);
    }
    Ok(outputs)
}
