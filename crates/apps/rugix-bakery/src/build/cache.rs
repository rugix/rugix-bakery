//! Content identities for artifact inputs and rule implementation resources.

use std::path::Path;
use std::process::Stdio;

use reportify::ResultExt;
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;

use crate::BakeryResult;

/// Hash a metadata-preserving archive without materializing it on disk.
///
/// GNU tar records ownership, modes, symlinks, device nodes, ACLs, and xattrs.
/// Access and change times are omitted because merely reading an input changes them.
pub fn artifact_hash(path: &Path) -> BakeryResult<String> {
    let mut command = tokio::process::Command::new("tar");
    command.args([
        "--create",
        "--file=-",
        "--format=pax",
        "--sort=name",
        "--numeric-owner",
        "--acls",
        "--selinux",
        "--xattrs",
        "--xattrs-include=*",
        "--sparse",
        "--sparse-version=0.0",
        "--pax-option=exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime",
    ]);
    if path
        .symlink_metadata()
        .whatever("unable to inspect artifact for hashing")?
        .is_dir()
    {
        command.arg("--directory").arg(path).arg(".");
    } else {
        command
            .arg("--directory")
            .arg(path.parent().expect("artifact paths are absolute"))
            .arg("--")
            .arg(path.file_name().expect("file artifact has a basename"));
    }
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .whatever("unable to start artifact hashing")?;
    let mut stdout = child.stdout.take().expect("stdout is piped");
    rugix_tasks::block_on(async {
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536];
        loop {
            let count = stdout
                .read(&mut buffer)
                .await
                .whatever("unable to hash artifact")?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        if !child
            .wait()
            .await
            .whatever("unable to wait for artifact hashing")?
            .success()
        {
            reportify::bail!("unable to archive artifact for hashing");
        }
        Ok(hex::encode(hasher.finalize()))
    })
}
