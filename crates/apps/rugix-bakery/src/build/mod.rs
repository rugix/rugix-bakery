//! Plan and execute repository-defined artifact builds.

use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use reportify::{bail, whatever, ResultExt};
use sidex::types::Str;
use xscript::{run, Run};

use crate::BakeryResult;

mod artifacts;
mod cache;
pub mod dependencies;
mod execute;
mod graph;
mod layers;
mod planner;
mod release;
mod runtime;
mod secrets;

pub use execute::Executor;
pub use planner::{validate_name, Artifact, ArtifactLocation, Plan, Planner, RunTask, Task};

/// Execute an invocation independently of system or layer construction.
pub fn build_invocation(
    project: &crate::project::ProjectRef,
    invocation: &Path,
    output: &Path,
) -> BakeryResult<()> {
    let invocation = crate::config::load_config(invocation)?;
    let plan = Planner::new(project.repositories()?)?
        .with_layers(project, None, &crate::oven::ParameterOverrides::new())
        .plan(&invocation, BTreeMap::new(), BTreeMap::new())?;
    let mut executor = Executor::for_project(project)?;
    let artifacts = executor.execute(&plan)?;
    publish(&artifacts, output, project.dir())
}

/// Publish named outputs only after all commands have succeeded.
pub fn publish(
    artifacts: &BTreeMap<Str, PathBuf>,
    output: &Path,
    project: &Path,
) -> BakeryResult<()> {
    let parent = output
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).whatever("unable to create build output parent")?;
    let staging = tempfile::Builder::new()
        .prefix(".assembly-")
        .tempdir_in(parent)
        .whatever("unable to create assembly publication directory")?;
    let destination = if output.exists() {
        fs::canonicalize(output).whatever("unable to resolve build output")?
    } else {
        fs::canonicalize(parent)
            .whatever("unable to resolve build output parent")?
            .join(
                output
                    .file_name()
                    .ok_or_else(|| whatever!("build output must name a directory"))?,
            )
    };
    let project = fs::canonicalize(project).whatever("unable to resolve project directory")?;
    if project.starts_with(&destination)
        || artifacts
            .values()
            .any(|source| source.starts_with(&destination))
    {
        bail!("build output must not contain the project or its source artifacts");
    }
    for (name, source) in artifacts {
        let filename = name.as_str();
        run!([
            "cp",
            "--archive",
            "--reflink=auto",
            "--sparse=always",
            "--",
            source,
            staging.path().join(filename)
        ])
        .whatever("unable to publish assembly artifact")?;
    }
    if output.exists() {
        fs::remove_dir_all(output).whatever("unable to replace previous build")?;
    }
    fs::rename(staging.path(), output).whatever("unable to publish build")?;
    Ok(())
}

/// Copy an exported file without truncating aliases of its source.
pub fn copy_file(source: &Path, destination: &Path) -> BakeryResult<()> {
    let source_metadata = fs::metadata(source).whatever("unable to inspect export source")?;
    match fs::metadata(destination) {
        Ok(metadata)
            if (metadata.dev(), metadata.ino())
                == (source_metadata.dev(), source_metadata.ino()) =>
        {
            return Ok(())
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error).whatever("unable to inspect export destination"),
    }
    if let Some(parent) = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).whatever("unable to create export parent directory")?;
    }
    fs::copy(source, destination).whatever("unable to export file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exporting a hardlink or absolute alias must preserve the source bytes.
    #[test]
    fn export_aliases_and_new_destinations() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source");
        let alias = directory.path().join("alias");
        fs::write(&source, "image contents").unwrap();
        fs::hard_link(&source, &alias).unwrap();
        copy_file(&source, &alias).unwrap();
        copy_file(&source, &source).unwrap();
        assert_eq!(fs::read(&source).unwrap(), b"image contents");
        let destination = directory.path().join("new/export");
        copy_file(&source, &destination).unwrap();
        assert_eq!(fs::read(destination).unwrap(), b"image contents");
    }

    /// Publication preserves port names and refuses destinations containing source data.
    #[test]
    fn publication_preserves_names_and_sources() {
        let directory = tempfile::tempdir().unwrap();
        let project = directory.path().join("project");
        fs::create_dir(&project).unwrap();
        let source = project.join("source");
        fs::write(&source, "image contents").unwrap();
        let artifacts = BTreeMap::from([
            ("image".into(), source.clone()),
            ("system.img".into(), source.clone()),
        ]);
        assert!(publish(&artifacts, &project, &project).is_err());
        assert_eq!(fs::read(&source).unwrap(), b"image contents");
        let output = project.join("build");
        publish(&artifacts, &output, &project).unwrap();
        assert_eq!(fs::read(output.join("image")).unwrap(), b"image contents");
        assert_eq!(
            fs::read(output.join("system.img")).unwrap(),
            b"image contents"
        );
    }
}
