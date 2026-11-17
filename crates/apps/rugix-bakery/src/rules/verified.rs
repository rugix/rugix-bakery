//! Boot packaging and A/B layout policies used by the core verified-boot compositions.
//!
//! Filesystem creation, hashing, signing and disk serialization remain separate rules.

use std::collections::BTreeSet;
use std::ffi::OsString;
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

/// Authenticated variables of one systemd-boot key set.
const SECURE_BOOT_KEY_FILES: [&str; 3] = ["PK.auth", "KEK.auth", "db.auth"];

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
    let mut loader = format!("default rugix-a.efi\ntimeout {}\neditor no\n", p.timeout);
    if install_secure_boot_keys(&esp)? {
        // `force` is the only mode in which the bundled systemd-boot 252 enrolls a key set
        // without user interaction. It still enrolls only in Setup Mode and reboots afterwards.
        loader.push_str("secure-boot-enroll force\n");
    }
    fs::write(esp.join("loader/loader.conf"), loader)
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
    check_rpi_boot_image_size()?;
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
    verify_rpi_output()
}

/// Verify a Raspberry Pi boot signature independently of the signer that produced it.
pub fn verify_rpi() -> BakeryResult<()> {
    check_rpi_boot_image_size()?;
    fs::copy(
        Path::new(INPUTS).join("signature"),
        Path::new(OUTPUTS).join("boot.sig"),
    )
    .whatever("unable to export signature")?;
    verify_rpi_output()
}

/// Verify the exported `boot.sig` against the `image` and `public-key` inputs.
fn verify_rpi_output() -> BakeryResult<()> {
    run(command("/usr/local/lib/rugix/rpi-eeprom-digest").args([
        "-i",
        "/run/rugix/bakery/work/inputs/image",
        "-k",
        "/run/rugix/bakery/work/inputs/public-key",
        "-v",
        "/run/rugix/bakery/work/outputs/boot.sig",
    ]))
}

/// Firmware loads the boot image into RAM; keep disk headroom outside it.
fn check_rpi_boot_image_size() -> BakeryResult<()> {
    if fs::metadata(Path::new(INPUTS).join("image"))
        .whatever("unable to inspect Raspberry Pi boot image")?
        .len()
        > 180_000_000
    {
        bail!("Raspberry Pi boot.img exceeds the 180 MB RAM disk budget");
    }
    Ok(())
}

/// Build the initial factory layout; Rugix bootstrap creates the spare system and data.
pub fn layout(rpi: bool) -> BakeryResult<()> {
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Parameters {
        secondary_config: bool,
    }
    let secondary = if rpi {
        false
    } else {
        parameters::<Parameters>()?.secondary_config
    };
    write_layout(rpi, secondary, Path::new(INPUTS), Path::new(OUTPUTS))
}

/// Export a factory layout and, for EFI, its correctly numbered image artifacts.
fn write_layout(rpi: bool, secondary: bool, inputs: &Path, outputs: &Path) -> BakeryResult<()> {
    let capacity = |name: &str| -> BakeryResult<u64> {
        Ok(fs::metadata(inputs.join(name))
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
    let config_size = capacity("config")?;
    let config = partition("config", config_size, true);
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
        let mut partitions = vec![config];
        if secondary {
            // Keep the staging area out of firmware ESP discovery until activation.
            partitions.push(partition("config-secondary", config_size, false));
        }
        partitions.push(system);
        let images = outputs.join("partitions");
        fs::create_dir(&images).whatever("unable to create partition artifacts")?;
        for (name, number) in [("config", 1), ("system", partitions.len())] {
            fs::copy(inputs.join(name), images.join(number.to_string()))
                .whatever("unable to export partition artifact")?;
        }
        partitions
    };
    let layout = ImageLayout {
        ty: Some(PartitionTableType::Gpt),
        partitions: Some(partitions.into()),
        raw_blobs: None,
    };
    fs::write(
        outputs.join("layout.json"),
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

/// Install supplied Secure Boot variables as the key set systemd-boot enrolls
/// automatically.
///
/// Returns whether the `secure-boot-keys` input contains variables.
fn install_secure_boot_keys(esp: &Path) -> BakeryResult<bool> {
    let source = Path::new(INPUTS).join("secure-boot-keys");
    let mut names = BTreeSet::new();
    for entry in fs::read_dir(&source).whatever("unable to read Secure Boot keys")? {
        names.insert(
            entry
                .whatever("unable to read Secure Boot keys")?
                .file_name(),
        );
    }
    if names.is_empty() {
        return Ok(false);
    }
    if names != SECURE_BOOT_KEY_FILES.map(OsString::from).into() {
        bail!("Secure Boot keys must consist of exactly PK.auth, KEK.auth, and db.auth");
    }
    let target = esp.join("loader/keys/auto");
    fs::create_dir_all(&target).whatever("unable to create Secure Boot key directory")?;
    for name in SECURE_BOOT_KEY_FILES {
        fs::copy(source.join(name), target.join(name))
            .whatever("unable to install Secure Boot key")?;
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek, SeekFrom};

    use super::*;

    #[test]
    fn efi_factory_partitions_match_artifacts() {
        // Both layouts assemble the root in the right slot and leave staging space empty.
        for secondary in [false, true] {
            let temp = tempfile::tempdir().unwrap();
            let inputs = temp.path().join("inputs");
            let outputs = temp.path().join("outputs");
            fs::create_dir(&inputs).unwrap();
            fs::create_dir(&outputs).unwrap();
            fs::write(inputs.join("config"), vec![0x43; 1024 * 1024]).unwrap();
            fs::write(inputs.join("system"), vec![0x53; 4096]).unwrap();
            write_layout(false, secondary, &inputs, &outputs).unwrap();
            let layout: ImageLayout =
                crate::config::load_config_or_json(&outputs.join("layout.json")).unwrap();
            let partitions = layout.partitions.unwrap();
            let names: Vec<_> = partitions
                .iter()
                .map(|p| p.name.as_deref().unwrap())
                .collect();
            assert_eq!(
                names,
                if secondary {
                    vec!["config", "config-secondary", "system-a"]
                } else {
                    vec!["config", "system-a"]
                }
            );
            if secondary {
                assert_eq!(partitions[0].size, partitions[1].size);
                assert!(partitions[1].ty.is_none());
                assert!(!outputs.join("partitions/2").exists());
            }
            let disk = outputs.join("factory.img");
            super::super::disk::image_gpt(
                &outputs.join("layout.json"),
                &outputs.join("partitions"),
                &disk,
                None,
            )
            .unwrap();
            let table = rugix_common::disk::PartitionTable::read(&disk).unwrap();
            let mut image = fs::File::open(disk).unwrap();
            for (index, expected) in [
                (0, vec![0x43; 1024 * 1024]),
                (partitions.len() - 1, vec![0x53; 4096]),
            ] {
                image
                    .seek(SeekFrom::Start(
                        table
                            .blocks_to_bytes(table.partitions[index].start)
                            .into_raw(),
                    ))
                    .unwrap();
                let mut bytes = vec![0; expected.len()];
                image.read_exact(&mut bytes).unwrap();
                assert_eq!(bytes, expected);
            }
            if secondary {
                image
                    .seek(SeekFrom::Start(
                        table.blocks_to_bytes(table.partitions[1].start).into_raw(),
                    ))
                    .unwrap();
                let mut bytes = vec![1; 1024 * 1024];
                image.read_exact(&mut bytes).unwrap();
                assert!(bytes.iter().all(|b| *b == 0));
            }
        }
    }
}
