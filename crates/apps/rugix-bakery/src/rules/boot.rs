//! EFI artifacts, initrds, and deterministic signatures.

use std::fs;
use std::path::Path;
use std::process::Stdio;

use std::os::unix::fs::PermissionsExt;

use reportify::{bail, ResultExt};
use serde::Deserialize;

use super::{check_rsa_key, command, copy_tree, epoch_command, parameters, run, signing_time};
use crate::BakeryResult;

pub fn select_efi(name: &str) -> BakeryResult<()> {
    #[derive(Deserialize)]
    struct Parameters {
        architecture: crate::config::systems::Architecture,
    }
    let p: Parameters = parameters()?;
    let suffix = match p.architecture {
        crate::config::systems::Architecture::Amd64 => "x64",
        crate::config::systems::Architecture::Arm64 => "aa64",
        _ => bail!("unsupported EFI architecture {:?}", p.architecture),
    };
    let (source, destination) = if name == "efi-stub" {
        (format!("linux{suffix}.efi.stub"), "stub.efi")
    } else {
        (format!("systemd-boot{suffix}.efi"), "systemd-boot.efi")
    };
    fs::copy(
        Path::new("/usr/lib/systemd/boot/efi").join(source),
        Path::new("/run/rugix/bakery/work/outputs").join(destination),
    )
    .whatever("unable to copy EFI executable")?;
    Ok(())
}

pub fn initrd() -> BakeryResult<()> {
    let version = fs::read_to_string("/run/rugix/bakery/work/inputs/kernel-version")
        .whatever("unable to read kernel version")?;
    let version = version.trim();
    if version.is_empty()
        || version.contains('/')
        || version.starts_with('-')
        || version.chars().any(char::is_whitespace)
    {
        bail!("expected one explicit kernel version");
    }
    copy_tree(
        Path::new("/run/rugix/bakery/work/inputs/rootfs"),
        Path::new("/run/rugix/bakery/work/outputs/.rootfs"),
    )?;
    run(epoch_command("chroot", super::source_date_epoch()?)?.args([
        "/run/rugix/bakery/work/outputs/.rootfs",
        "mkinitramfs",
        "-o",
        "/tmp/rugix-initrd",
        version,
    ]))?;
    fs::copy(
        "/run/rugix/bakery/work/outputs/.rootfs/tmp/rugix-initrd",
        "/run/rugix/bakery/work/outputs/initrd",
    )
    .whatever("unable to copy initrd")?;
    fs::remove_dir_all("/run/rugix/bakery/work/outputs/.rootfs")
        .whatever("unable to remove initrd workspace")
}

pub fn uki() -> BakeryResult<()> {
    run(command("ukify").args([
        "build",
        "--config=/dev/null",
        "--stub=/run/rugix/bakery/work/inputs/stub",
        "--linux=/run/rugix/bakery/work/inputs/kernel",
        "--initrd=/run/rugix/bakery/work/inputs/initrd",
        "--cmdline=@/run/rugix/bakery/work/inputs/cmdline",
        "--os-release=@/run/rugix/bakery/work/inputs/os-release",
        "--output=/run/rugix/bakery/work/outputs/boot.efi",
    ]))
}

pub fn sign_efi() -> BakeryResult<()> {
    let p = signing_time()?;
    check_rsa_key()?;
    run(command("osslsigncode")
        .args([
            "sign",
            "-h",
            "sha256",
            "-certs",
            "/run/rugix/bakery/work/inputs/certificate",
            "-key",
            "/run/rugix/bakery/work/secrets/key",
            "-time",
        ])
        .arg(p.to_string())
        .args([
            "-in",
            "/run/rugix/bakery/work/inputs/image",
            "-out",
            "/run/rugix/bakery/work/outputs/signed.efi",
        ]))?;
    run(command("sbverify").args([
        "--cert",
        "/run/rugix/bakery/work/inputs/certificate",
        "/run/rugix/bakery/work/outputs/signed.efi",
    ]))
}

/// Verify an EFI signature independently of the signer that produced it.
///
/// The signature is moved from the signer's output onto the unsigned image that the
/// graph built, so the exported image is that image with the returned signature.
/// Verification fails if the signature covers a different Authenticode digest.
pub fn verify_efi() -> BakeryResult<()> {
    let signature = Path::new("/run/rugix/bakery/work/outputs/.signature");
    let image = Path::new("/run/rugix/bakery/work/outputs/signed.efi");
    run(command("sbattach")
        .arg("--detach")
        .arg(signature)
        .arg("/run/rugix/bakery/work/inputs/signed"))?;
    fs::copy("/run/rugix/bakery/work/inputs/image", image)
        .whatever("unable to copy unsigned EFI image")?;
    fs::set_permissions(image, fs::Permissions::from_mode(0o644))
        .whatever("unable to make EFI image writable")?;
    run(command("sbattach")
        .arg("--attach")
        .arg(signature)
        .arg(image)
        .stdout(Stdio::null()))?;
    fs::remove_file(signature).whatever("unable to remove detached signature")?;
    run(command("sbverify")
        .args(["--cert", "/run/rugix/bakery/work/inputs/certificate"])
        .arg(image))
}
