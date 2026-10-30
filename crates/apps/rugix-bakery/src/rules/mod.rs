//! Rust implementations of core repository rules.
//!
//! Definitions and defaults remain in repositories. The executor treats these
//! entry points as ordinary commands using the standard rule mount protocol.

use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use reportify::{bail, ResultExt};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};

use crate::BakeryResult;

mod filesystems;
mod layout;
mod sbom;
mod standard;

/// Execute a built-in implementation using mounted rule inputs and outputs.
#[tracing::instrument]
pub fn execute(name: &str) -> BakeryResult<()> {
    match name {
        "image-ext4" => filesystems::ext4(),
        "image-fat" => filesystems::fat(),
        "image-squashfs" => filesystems::squashfs(),
        "bundle-rugix" => {
            fs::create_dir("/run/rugix/bakery/work/outputs/.bundle")
                .whatever("unable to create bundle workspace")?;
            fs::copy(
                "/run/rugix/bakery/work/inputs/manifest",
                "/run/rugix/bakery/work/outputs/.bundle/rugix-bundle.toml",
            )
            .whatever("unable to copy bundle manifest")?;
            std::os::unix::fs::symlink(
                "/run/rugix/bakery/work/inputs/payloads",
                "/run/rugix/bakery/work/outputs/.bundle/payloads",
            )
            .whatever("unable to link bundle payloads")?;
            run(command("rugix-bundler").args([
                "bundle",
                "/run/rugix/bakery/work/outputs/.bundle",
                "/run/rugix/bakery/work/outputs/update.rugixb",
            ]))?;
            fs::remove_dir_all("/run/rugix/bakery/work/outputs/.bundle")
                .whatever("unable to remove bundle workspace")
        }
        "image-layout" => standard::image_layout(),
        "initialize-boot" => standard::initialize_boot(),
        "prepare-boot" => standard::prepare_boot(),
        "assemble-disk" => standard::assemble_disk(),
        "sbom" => sbom::execute(),
        "archive-tree" => standard::archive_tree(),
        _ => bail!("unknown internal rule {name:?}"),
    }
}

/// Decode the executor's parameter transport into a rule-specific type.
fn parameters<T: DeserializeOwned>() -> BakeryResult<T> {
    let bytes = fs::read("/run/rugix/bakery/work/parameters.json")
        .whatever("unable to read rule parameters")?;
    serde_json::from_slice(&bytes).whatever("invalid core rule parameters")
}

fn command(program: &str) -> Command {
    let mut command = Command::new(program);
    command.env("LC_ALL", "C").env("TZ", "UTC");
    command
}

/// Read the source epoch supplied by the build executor.
fn source_date_epoch() -> BakeryResult<u64> {
    Ok(crate::build::ExecutionContext::resolve(None)?.source_date_epoch)
}

fn epoch_command(program: &str, epoch: u64) -> BakeryResult<Command> {
    if epoch > u64::from(u32::MAX) {
        bail!("source-date-epoch must be between 0 and 4294967295");
    }
    let mut cmd = command(program);
    cmd.env("SOURCE_DATE_EPOCH", epoch.to_string());
    Ok(cmd)
}

fn run(command: &mut Command) -> BakeryResult<()> {
    let program = command.get_program().to_owned();
    if !command
        .status()
        .whatever_with(|_| format!("unable to execute {program:?}"))?
        .success()
    {
        bail!("core rule command {program:?} failed");
    }
    Ok(())
}

fn output(command: &mut Command) -> BakeryResult<Output> {
    let program = command.get_program().to_owned();
    let output = command
        .stderr(Stdio::inherit())
        .output()
        .whatever_with(|_| format!("unable to execute {program:?}"))?;
    if !output.status.success() {
        bail!("core rule command {program:?} failed");
    }
    Ok(output)
}

fn options(value: &str) -> BakeryResult<Vec<String>> {
    shlex::split(value).ok_or_else(|| reportify::whatever!("invalid quoting in formatter options"))
}

fn digest(reader: &mut impl Read) -> BakeryResult<[u8; 32]> {
    let mut hash = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let count = reader
            .read(&mut buffer)
            .whatever("unable to hash rule input")?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hash.finalize().into())
}

fn file_digest(path: &Path) -> BakeryResult<[u8; 32]> {
    digest(&mut fs::File::open(path).whatever("unable to open rule input")?)
}

fn sparse_file(path: &str, size: u64) -> BakeryResult<()> {
    fs::File::create(path)
        .whatever("unable to create image")?
        .set_len(size)
        .whatever("unable to size image")
}

/// Copy a tree without losing ownership, hard links, or extended attributes.
fn copy_tree(source: &Path, destination: &Path) -> BakeryResult<()> {
    run(command("cp")
        .args(["--archive", "--reflink=auto", "--"])
        .arg(source)
        .arg(destination))
}
