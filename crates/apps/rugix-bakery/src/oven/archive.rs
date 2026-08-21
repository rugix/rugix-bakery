//! Metadata-preserving archive operations for Bakery layers.
//!
//! The module centralizes the GNU tar options required to retain the filesystem
//! metadata represented by Bakery layers and to produce deterministic archives.
//! [`create`] and [`extract`] are its entry points.

use std::path::Path;

use reportify::ResultExt;
use xscript::{cmd_os, vars, ParentEnv, Run};

use crate::BakeryResult;

/// Extracts a layer archive while restoring its filesystem metadata.
#[tracing::instrument(
    level = "debug",
    skip_all,
    fields(source = %source.display(), destination = %destination.display())
)]
pub(crate) fn extract(source: &Path, destination: &Path) -> BakeryResult<()> {
    ParentEnv
        .run(cmd_os!(
            "tar",
            "--extract",
            "--file",
            source,
            "--directory",
            destination,
            "--same-owner",
            "--same-permissions",
            "--acls",
            "--selinux",
            "--xattrs",
            "--xattrs-include=*",
        ))
        .whatever("failed to extract metadata-preserving archive")?;
    Ok(())
}

/// Creates a deterministic layer archive while preserving filesystem metadata.
#[tracing::instrument(
    level = "debug",
    skip_all,
    fields(source = %source.display(), destination = %destination.display())
)]
pub(crate) fn create(
    source: &Path,
    destination: &Path,
    source_date_epoch: Option<i64>,
) -> BakeryResult<()> {
    let mut tar_command = cmd_os!(
        "tar",
        "--create",
        "--file",
        destination,
        "--directory",
        source,
        "--format=pax",
        "--sort=name",
        "--numeric-owner",
        "--atime-preserve=system",
        "--acls",
        "--selinux",
        "--xattrs",
        "--xattrs-include=*",
        "--sparse",
        // Newer PAX sparse formats encode a process-specific path.
        "--sparse-version=0.0",
        "--pax-option=exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime",
    );
    if let Some(source_date_epoch) = source_date_epoch {
        tar_command.add_arg("--clamp-mtime");
        tar_command.add_arg(format!("--mtime=@{source_date_epoch}"));
    }
    tar_command.add_arg(".");
    ParentEnv
        .run(tar_command.with_vars(vars! {
            LC_ALL = "C",
        }))
        .whatever("failed to create metadata-preserving archive")?;
    Ok(())
}
