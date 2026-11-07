//! EFI artifacts, initrds, and deterministic signatures.

use std::fs;
use std::path::Path;
use std::process::Stdio;

use reportify::{bail, ResultExt};
use serde::Deserialize;

use super::{command, copy_tree, epoch_command, parameters, run};
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

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Signing {
    signing_time: String,
}

fn signing() -> BakeryResult<u64> {
    let p: Signing = parameters()?;
    let time = if p.signing_time.is_empty() {
        super::source_date_epoch()?
    } else {
        p.signing_time
            .parse::<u64>()
            .whatever("invalid signing-time")?
    };
    if time > 2524607999 {
        bail!("signing-time must be between 0 and 2524607999");
    }
    run(command("openssl")
        .args([
            "rsa",
            "-in",
            "/run/rugix/bakery/work/secrets/key",
            "-check",
            "-noout",
        ])
        .stdout(Stdio::null()))?;
    Ok(time)
}

pub fn sign_efi() -> BakeryResult<()> {
    let p = signing()?;
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

pub fn sign_bundle() -> BakeryResult<()> {
    let time = signing()?;
    let bundle = Path::new("/run/rugix/bakery/work/inputs/bundle");
    let metadata =
        rugix_bundle::signed_metadata(bundle).whatever("unable to read signed bundle metadata")?;
    let certificate = fs::read("/run/rugix/bakery/work/inputs/certificate")
        .whatever("unable to read bundle signer certificate")?;
    let key = zeroize::Zeroizing::new(
        fs::read("/run/rugix/bakery/work/secrets/key")
            .whatever("unable to read bundle signer key")?,
    );
    let signer = rugix_pki::CmsSignerBuilder::new(&certificate, &key)
        .whatever("unable to create bundle signer")?
        .with_rsa_mode(rugix_pki::RsaSignatureMode::Pkcs1v15)
        .with_signing_time(std::time::UNIX_EPOCH + std::time::Duration::from_secs(time))
        .build()
        .whatever("unable to build bundle signer")?;
    let signature = signer.sign(&metadata).whatever("unable to sign bundle")?;
    rugix_bundle::add_bundle_signature(
        bundle,
        signature,
        Path::new("/run/rugix/bakery/work/outputs/update.rugixb"),
    )
    .whatever("unable to append bundle signature")?;
    super::write_bundle_hash()
}
