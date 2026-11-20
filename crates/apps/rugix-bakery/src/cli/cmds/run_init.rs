//! The `init` command.

use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;

use reportify::{bail, ResultExt};

use rugix_common::fsutils::copy_recursive;

use crate::cli::{args, current_dir};
use crate::config::repositories::GitSourceConfig;
use crate::config::templates::TemplateManifest;
use crate::project::git::{check_out_git_source, compute_git_source_id};
use crate::{paths, BakeryResult};

/// Run the `init` command.
pub fn run(cmd: &args::InitCommand) -> BakeryResult<()> {
    let cwd = current_dir()?;

    // Guard against re-initialization before materializing external sources.
    //
    // The guard is only applied when actually initializing: listing the
    // available templates is read-only and must work in initialized projects.
    if cmd.template.is_some() && cwd.join("rugix-bakery.toml").exists() {
        bail!("Project has already been initialized.");
    }

    let manifest_root = match (cmd.template_repo.as_ref(), cmd.template_dir.clone()) {
        // local template repo with fallback to bundled templates
        (None, dir) => dir.unwrap_or(paths::templates_dir()),
        // remote template repo
        (Some(url), manifest_dir) => {
            let root = materialize_template_source(url, cmd.template_rev.as_deref(), &cwd, true)
                .whatever_with(|_| format!("unable to materialize template repository {url}"))?;
            manifest_dir
                .clone()
                .map_or(root.clone(), |subdir| root.join(subdir))
        }
    };

    if !manifest_root.is_dir() {
        bail!("Template directory {manifest_root:?} does not exist.");
    }
    let manifest_path = manifest_root.join("rugix-templates.toml");

    if !manifest_path.exists() {
        bail!("Template directory {manifest_root:?} does not contain `rugix-templates.toml`.");
    }

    let raw_manifest = fs::read_to_string(&manifest_path).whatever_with(|_| {
        format!(
            "error reading templates list from {}",
            manifest_path.display()
        )
    })?;

    let templates: TemplateManifest =
        sidex::formats::toml::from_str(&raw_manifest).whatever_with(|_| {
            format!(
                "error parsing templates list from {}",
                manifest_path.display()
            )
        })?;

    let Some(template_name) = &cmd.template else {
        rugix_cli::suspend(|| {
            eprintln!("{}\n", "Available Templates:".bold());
            let mut names = templates
                .keys()
                .map(|name| name.as_str().to_owned())
                .collect::<Vec<_>>();
            names.sort();
            for name in names {
                let Some(info) = templates.get(name.as_str()) else {
                    continue;
                };
                eprintln!(
                    "  {}:\n    {}",
                    name.blue(),
                    info.description.trim().bright_black()
                );
            }
        });
        return Ok(());
    };

    let Some(info) = templates.get(template_name.as_str()) else {
        let mut names = templates
            .keys()
            .map(|name| name.as_str().to_owned())
            .collect::<Vec<_>>();
        names.sort_unstable();
        bail!(
            "Unknown template `{template_name}`. Available templates: {}",
            names.join(", ")
        );
    };
    let template_dir = manifest_root.join(info.path.as_deref().unwrap_or(template_name));
    copy_recursive(template_dir, cwd).whatever("error copying template to project directory")?;
    Ok(())
}

/// Materialize an external template repository and return its root directory.
///
/// The repository is cloned (and subsequently fetched) into
/// `<root>/.rugix/templates/<source-id>`, mirroring how project repositories
/// are cached.
///
/// Note that the source id hashes the repository URL (and `dir`) only, so
/// different revisions of the same repository share a cache directory. The
/// repository is re-fetched and checked out to the requested revision on
/// every run.
fn materialize_template_source(
    url: &str,
    rev: Option<&str>,
    root: &Path,
    fetch: bool,
) -> BakeryResult<PathBuf> {
    let config = GitSourceConfig {
        url: url.into(),
        tag: None,
        branch: None,
        rev: rev.map(Into::into),
        dir: None,
    };
    let id = compute_git_source_id(&config);
    let path = root.join(".rugix/templates").join(id.as_str());
    check_out_git_source(&config, &path, fetch)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    /// Restores the process working directory on drop, undoing any change
    /// made by the test.
    struct CwdGuard(PathBuf);

    impl CwdGuard {
        fn new() -> Self {
            Self(std::env::current_dir().unwrap())
        }
    }

    impl Drop for CwdGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }

    /// Parse a manifest and resolve a template directory.
    fn resolve(manifest: &str, name: &str) -> Option<PathBuf> {
        let templates: TemplateManifest = sidex::formats::toml::from_str(manifest).unwrap();
        let info = templates.get(name)?;
        Some(Path::new("/source").join(info.path.as_deref().unwrap_or(name)))
    }

    #[test]
    fn template_path_defaults_to_name() {
        assert_eq!(
            resolve(
                r#"
[template-a]
description = "template-a-test"
"#,
                "template-a",
            )
            .unwrap(),
            Path::new("/source/template-a")
        );
    }

    #[test]
    fn template_path_override() {
        assert_eq!(
            resolve(
                r#"
[template-a]
description = "template-a-test"
path = "templates/template-a"
"#,
                "template-a",
            )
            .unwrap(),
            Path::new("/source/templates/template-a")
        );
    }

    #[test]
    fn init_from_local_template_dir() {
        let source = tempfile::tempdir().unwrap();
        let project = tempfile::tempdir().unwrap();
        fs::create_dir_all(
            source
                .path()
                .join("test-template")
                .join("layers")
                .join("test"),
        )
        .unwrap();
        fs::write(
            source.path().join("rugix-templates.toml"),
            r#"
[test-template]
description = "Test template."
"#,
        )
        .unwrap();
        fs::write(
            source
                .path()
                .join("test-template")
                .join("rugix-bakery.toml"),
            "",
        )
        .unwrap();

        let _cwd_guard = CwdGuard::new();
        std::env::set_current_dir(project.path()).unwrap();

        let cmd = args::InitCommand {
            template: Some("test-template".to_owned()),
            template_repo: None,
            template_dir: Some(source.path().to_path_buf()),
            template_rev: None,
        };

        assert!(run(&cmd).is_ok());
        // running the init twice produces an error
        assert!(run(&cmd).is_err());

        assert!(project.path().join("rugix-bakery.toml").exists());
        assert!(project.path().join("layers").join("test").exists());
    }
}
