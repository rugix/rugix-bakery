//! Boot packaging and A/B layout policies used by the core verified-boot compositions.
//!
//! Filesystem creation, hashing, signing and disk serialization remain separate rules.

use std::fs;
use std::path::Path;

use reportify::{bail, ResultExt};
use serde::Deserialize;

use crate::config::images::{ImageLayout, ImagePartition, PartitionTableType};
use crate::config::systems::Architecture;
use crate::BakeryResult;

use super::{command, copy_tree, output, parameters, run};

const INPUTS: &str = "/run/rugix/bakery/work/inputs";
const OUTPUTS: &str = "/run/rugix/bakery/work/outputs";

/// Select exactly one installed kernel, avoiding a silently changing boot ABI.
pub fn kernel() -> BakeryResult<()> {
    #[derive(Deserialize)]
    struct Parameters {
        version: String,
        flavor: String,
    }
    let p: Parameters = parameters()?;
    let root = Path::new(INPUTS).join("rootfs");
    let mut versions = Vec::new();
    for entry in fs::read_dir(root.join("lib/modules")).whatever("unable to read kernel modules")? {
        let entry = entry.whatever("unable to inspect installed kernel")?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| reportify::whatever!("invalid kernel version"))?;
        if entry
            .file_type()
            .whatever("unable to inspect kernel modules")?
            .is_dir()
            && (p.version.is_empty() || name == p.version)
            && name.ends_with(&p.flavor)
        {
            versions.push(name);
        }
    }
    if versions.len() != 1 {
        bail!("kernel selection must match exactly one installed kernel; set version or flavor");
    }
    let version = &versions[0];
    fs::copy(
        root.join(format!("boot/vmlinuz-{version}")),
        Path::new(OUTPUTS).join("kernel"),
    )
    .whatever("unable to export selected kernel")?;
    fs::write(Path::new(OUTPUTS).join("version"), format!("{version}\n"))
        .whatever("unable to export kernel version")
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Platform {
    Efi,
    Rpi,
}

/// Authenticate the verity root through the boot artifact's command line.
pub fn cmdline() -> BakeryResult<()> {
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Parameters {
        platform: Platform,
        extra: String,
        panic_timeout: u64,
    }
    let p: Parameters = parameters()?;
    let hash = fs::read_to_string(Path::new(INPUTS).join("root-hash"))
        .whatever("unable to read root hash")?;
    let hash = hash.trim();
    if hash.len() != 64 || !hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("verified boot requires a SHA-256 root hash");
    }
    let value = |name: &str| -> BakeryResult<u64> {
        fs::read_to_string(Path::new(INPUTS).join(name))
            .whatever("unable to read verity geometry")?
            .trim()
            .parse::<u64>()
            .whatever("invalid verity geometry")
    };
    let offset = value("hash-offset")?;
    let size = value("data-size")?;
    if size == 0
        || size % 4096 != 0
        || offset < size
        || offset % 4096 != 0
        || offset > i64::MAX as u64
    {
        bail!("verified boot requires aligned SHA-256/4096 verity geometry");
    }
    let blocks = size / 4096;
    if p.extra.split_ascii_whitespace().any(|arg| {
        matches!(arg, "rw" | "ro" | "break" | "rd.break")
            || [
                "break=",
                "rd.break=",
                "root=",
                "rootfstype=",
                "init=",
                "rdinit=",
                "rugix.",
                "panic=",
            ]
            .iter()
            .any(|prefix| arg.starts_with(prefix))
    }) || p.extra.contains(['\n', '\r', '\0'])
    {
        bail!("extra command line must not override verified-root arguments or enable an initrd shell");
    }
    let platform = match p.platform {
        Platform::Efi => "efi",
        Platform::Rpi => "rpi",
    };
    fs::write(Path::new(OUTPUTS).join("cmdline"),
        format!("root=/dev/mapper/root rootfstype=squashfs ro init=/usr/bin/rugix-ctrl panic={} rugix.platform={platform} rugix.verity-root-hash={hash} rugix.verity-hash-offset={offset} rugix.verity-data-blocks={blocks} systemd.gpt_auto=0 rd.systemd.gpt_auto=0 {}\n", p.panic_timeout, p.extra))
        .whatever("unable to write authenticated command line")
}

/// Prepare a systemd-boot ESP without giving callers mutable boot arguments.
pub fn efi_esp() -> BakeryResult<()> {
    #[derive(Deserialize)]
    struct Parameters {
        architecture: Architecture,
        timeout: u64,
    }
    let p: Parameters = parameters()?;
    let suffix = match p.architecture {
        Architecture::Amd64 => "X64",
        Architecture::Arm64 => "AA64",
        _ => bail!("unsupported EFI architecture"),
    };
    let esp = Path::new(OUTPUTS).join("esp");
    for dir in ["EFI/BOOT", "EFI/Linux", "loader", ".rugix"] {
        fs::create_dir_all(esp.join(dir)).whatever("unable to create EFI directory")?;
    }
    fs::copy(
        Path::new(INPUTS).join("bootloader"),
        esp.join(format!("EFI/BOOT/BOOT{suffix}.EFI")),
    )
    .whatever("unable to install signed bootloader")?;
    fs::copy(
        Path::new(INPUTS).join("uki"),
        esp.join("EFI/Linux/rugix-a.efi"),
    )
    .whatever("unable to install signed UKI")?;
    fs::write(esp.join(".rugix/bootstrap"), "").whatever("unable to create bootstrap marker")?;
    fs::write(
        esp.join("loader/loader.conf"),
        format!("default rugix-a.efi\ntimeout {}\neditor no\n", p.timeout),
    )
    .whatever("unable to write systemd-boot configuration")
}

/// Supply all boot inputs inside the firmware-authenticated RAM disk.
pub fn rpi_boot() -> BakeryResult<()> {
    let source = Path::new(INPUTS).join("boot");
    let boot = Path::new(OUTPUTS).join("boot");
    fs::create_dir(&boot).whatever("unable to create Raspberry Pi boot tree")?;
    for name in ["start4.elf", "fixup4.dat"] {
        fs::copy(source.join(name), boot.join(name))
            .whatever("unable to copy Raspberry Pi firmware")?;
    }
    for entry in fs::read_dir(&source).whatever("unable to read Raspberry Pi firmware")? {
        let entry = entry.whatever("unable to read Raspberry Pi firmware entry")?;
        if entry.path().extension().is_some_and(|ext| ext == "dtb") {
            fs::copy(entry.path(), boot.join(entry.file_name()))
                .whatever("unable to copy device tree")?;
        }
    }
    copy_tree(&source.join("overlays"), &boot.join("overlays"))?;
    for (input, name) in [
        ("kernel", "kernel8.img"),
        ("initrd", "initrd"),
        ("cmdline", "cmdline.txt"),
        ("config", "config.txt"),
    ] {
        fs::copy(Path::new(INPUTS).join(input), boot.join(name))
            .whatever("unable to copy Raspberry Pi boot input")?;
    }
    Ok(())
}

/// Keep tryboot state outside the immutable, signed boot partitions.
pub fn rpi_config() -> BakeryResult<()> {
    let config = Path::new(OUTPUTS).join("config");
    copy_tree(&Path::new(INPUTS).join("config"), &config)?;
    fs::create_dir_all(config.join(".rugix")).whatever("unable to create bootstrap directory")?;
    fs::write(config.join(".rugix/bootstrap"), "").whatever("unable to create bootstrap marker")?;
    fs::write(
        config.join("autoboot.txt"),
        "[all]\ntryboot_a_b=1\nboot_partition=2\n[tryboot]\nboot_partition=3\n",
    )
    .whatever("unable to initialize tryboot")
}

/// Sign with the upstream Raspberry Pi tool, enforcing its RSA-2048 bootloader contract.
pub fn sign_rpi() -> BakeryResult<()> {
    // Firmware loads this image into RAM; keep disk headroom outside it.
    if fs::metadata(Path::new(INPUTS).join("image"))
        .whatever("unable to inspect Raspberry Pi boot image")?
        .len()
        > 180_000_000
    {
        bail!("Raspberry Pi boot.img exceeds the 180 MB RAM disk budget");
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Parameters {
        signing_time: String,
    }
    let p: Parameters = parameters()?;
    let epoch = rpi_signing_time(&p.signing_time)?;
    validate_rpi_key()?;
    run(command("/usr/local/lib/rugix/rpi-eeprom-digest")
        .env("SOURCE_DATE_EPOCH", epoch.to_string())
        .args([
            "-i",
            "/run/rugix/bakery/work/inputs/image",
            "-k",
            "/run/rugix/bakery/work/secrets/key",
            "-o",
            "/run/rugix/bakery/work/outputs/boot.sig",
        ]))?;
    run(command("/usr/local/lib/rugix/rpi-eeprom-digest").args([
        "-i",
        "/run/rugix/bakery/work/inputs/image",
        "-k",
        "/run/rugix/bakery/work/inputs/public-key",
        "-v",
        "/run/rugix/bakery/work/outputs/boot.sig",
    ]))
}

/// Build the initial factory layout; Rugix bootstrap creates the spare system and data.
pub fn layout(rpi: bool) -> BakeryResult<()> {
    let capacity = |name: &str| -> BakeryResult<u64> {
        Ok(fs::metadata(Path::new(INPUTS).join(name))
            .whatever("unable to inspect partition image")?
            .len())
    };
    let partition = |name: &str, size: u64, efi: bool| ImagePartition {
        name: Some(name.into()),
        size: Some(sidex::serde::Serde(byte_calc::NumBytes::new(size))),
        filesystem: None,
        root: None,
        bootable: None,
        ty: if efi {
            Some(sidex::serde::Serde(rugix_common::disk::gpt::gpt_types::EFI))
        } else {
            None
        },
    };
    let config = partition("config", capacity("config")?, true);
    let system = partition("system-a", capacity("system")?, false);
    let partitions = if rpi {
        let boot = capacity("boot")?;
        vec![
            config,
            partition("boot-a", boot, true),
            partition("boot-b", boot, true),
            system,
        ]
    } else {
        vec![config, system]
    };
    let layout = ImageLayout {
        ty: Some(PartitionTableType::Gpt),
        partitions: Some(partitions.into()),
        raw_blobs: None,
    };
    fs::write(
        Path::new(OUTPUTS).join("layout.json"),
        serde_json::to_vec_pretty(&layout).whatever("unable to encode A/B layout")?,
    )
    .whatever("unable to write A/B layout")
}

/// Require an RSA-2048 public key matching the mounted private signing key.
fn validate_rpi_key() -> BakeryResult<()> {
    let public = Path::new(INPUTS).join("public-key");
    let modulus = output(
        command("openssl")
            .args(["rsa", "-pubin", "-noout", "-modulus", "-in"])
            .arg(&public),
    )?;
    if modulus
        .stdout
        .strip_prefix(b"Modulus=")
        .is_none_or(|value| value.trim_ascii().len() != 512)
    {
        bail!("Raspberry Pi boot signing requires an RSA-2048 key");
    }
    let private_public = output(command("openssl").args([
        "pkey",
        "-pubout",
        "-outform",
        "DER",
        "-in",
        "/run/rugix/bakery/work/secrets/key",
    ]))?;
    let expected_public = output(
        command("openssl")
            .args(["pkey", "-pubin", "-outform", "DER", "-in"])
            .arg(&public),
    )?;
    if private_public.stdout != expected_public.stdout {
        bail!("Raspberry Pi signing key does not match its public key");
    }
    Ok(())
}

/// Resolve the reproducible timestamp used by Raspberry Pi boot signatures.
fn rpi_signing_time(value: &str) -> BakeryResult<u64> {
    if value.is_empty() {
        super::source_date_epoch()
    } else {
        value.parse::<u64>().whatever("invalid signing-time")
    }
}
