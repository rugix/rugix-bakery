//! Execute rules with Bakery's tooling and private artifact mounts.

use std::ffi::OsString;
use std::io::{self, IsTerminal};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

use reportify::{bail, ResultExt};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::envs::BindMount;
use crate::utils::command::execute_with_terminal;
use crate::BakeryResult;

use super::cache::artifact_hash;

/// Identify the immutable Bakery image and executable, including development overrides.
/// Without a resolved image ID, shared-runtime commands cannot safely reuse cached
/// outputs.
pub fn identity() -> BakeryResult<Option<String>> {
    let Some(image) = std::env::var("RUGIX_BAKERY_IMAGE").ok().filter(|image| {
        let digest = image
            .rsplit_once("sha256:")
            .map_or(image.as_str(), |(_, id)| id);
        digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
    }) else {
        return Ok(None);
    };
    let executable = std::env::current_exe().whatever("unable to locate Bakery executable")?;
    let mut key = Sha256::new();
    key.update(image.as_bytes());
    key.update(artifact_hash(&executable)?.as_bytes());
    Ok(Some(hex::encode(key.finalize())))
}

/// Bind rule resources in a child mount namespace without constructing an environment.
pub fn run(
    arguments: &[OsString],
    mounts: &[BindMount],
    environment: &[(&str, Option<&str>)],
) -> BakeryResult<ExitStatus> {
    let mut command = Command::new("rugix-isolate");
    command.args(["--supervise", "--", "unshare"]);
    for (name, value) in environment {
        match value {
            Some(value) => {
                command.env(name, value);
            }
            None => {
                command.env_remove(name);
            }
        }
    }
    command.args([
        "--mount",
        "--propagation",
        "private",
        "--",
        "/bin/sh",
        "-eu",
        "-c",
        include_str!("../../assets/run-rule"),
        "run-rule",
    ]);
    for mount in mounts {
        command
            .arg(mount.options()?)
            .arg(&mount.source)
            .arg(&mount.target);
    }
    command.arg("--").args(arguments).stdin(Stdio::inherit());
    let terminal = io::stdin().is_terminal() && io::stdout().is_terminal();
    Ok(execute_with_terminal(&mut command, terminal)?.status)
}

/// Expose every project submount explicitly so both runtimes make it read-only.
pub fn project_mounts(project: &Path) -> BakeryResult<Vec<BindMount>> {
    let output = Command::new("findmnt")
        .args(["--json", "--submounts", "--output", "TARGET", "--target"])
        .arg(project)
        .output()
        .whatever("unable to inspect project mounts")?;
    if !output.status.success() {
        bail!("unable to inspect project mounts");
    }
    let tree: MountTree =
        serde_json::from_slice(&output.stdout).whatever("unable to decode project mounts")?;
    let mut mounts = vec![BindMount {
        source: project.to_owned(),
        target: "/run/rugix/bakery/work/project".into(),
        read_only: true,
    }];
    let mut pending = tree.filesystems;
    while let Some(point) = pending.pop() {
        if point.target != project {
            if let Ok(relative) = point.target.strip_prefix(project) {
                mounts.push(BindMount {
                    source: point.target.clone(),
                    target: Path::new("/run/rugix/bakery/work/project").join(relative),
                    read_only: true,
                });
            }
        }
        pending.extend(point.children);
    }
    // Parent mounts must be installed before their children.
    mounts.sort_by_key(|mount| mount.target.components().count());
    Ok(mounts)
}

#[derive(Deserialize)]
struct MountTree {
    filesystems: Vec<MountPoint>,
}

#[derive(Deserialize)]
struct MountPoint {
    target: PathBuf,
    #[serde(default)]
    children: Vec<MountPoint>,
}
