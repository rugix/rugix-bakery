//! Disk geometry, boot-flow preparation, and assembly for system pipelines.

use std::fs::{self, File};
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

use reportify::{bail, ResultExt};
use rugix_common::disk::{DiskId, PartitionTable};
use rugix_common::utils::ascii_numbers;
use serde::Deserialize;

use crate::config::images::ImageLayout;
use crate::config::systems::{Architecture, Target};
use crate::oven::targets::generic_grub_efi::initialize_grub;
use crate::oven::targets::rpi_tryboot::initialize_tryboot;
use crate::oven::targets::rpi_uboot::initialize_uboot;
use crate::BakeryResult;

use super::{command, copy_tree, parameters, run};

/// Create an unpopulated disk with resolved geometry and raw firmware regions.
pub fn image_layout() -> BakeryResult<()> {
    #[derive(Deserialize)]
    struct Parameters {
        size: u64,
    }
    let p: Parameters = parameters()?;
    let layout: ImageLayout =
        crate::config::load_config_or_json(Path::new("/run/rugix/bakery/work/inputs/layout"))?;
    let mut table = super::layout::compute_partition_table(
        &layout,
        Path::new("/run/rugix/bakery/work/inputs/roots"),
        Path::new("/run/rugix/bakery/work/inputs/layer"),
    )?;
    if p.size != 0 {
        if p.size % 512 != 0 || p.size < table.size().into_raw() {
            bail!("configured image size is too small or not block-aligned");
        }
        table.disk_size = (p.size / 512).into();
        table.validate().whatever("invalid explicit disk size")?;
    }
    let image = Path::new("/run/rugix/bakery/work/outputs/layout.img");
    File::create(image)
        .whatever("unable to create disk skeleton")?
        .set_len(table.size().into_raw())
        .whatever("unable to size disk skeleton")?;
    table
        .write(image)
        .whatever("unable to write partition table")?;
    if let Some(blobs) = &layout.raw_blobs {
        super::layout::write_raw_blobs(
            blobs,
            Path::new("/run/rugix/bakery/work/inputs/layer"),
            image,
        )?;
    }
    Ok(())
}

/// Install boot files before computing their filesystem capacity.
pub fn initialize_boot() -> BakeryResult<()> {
    let p: BootParameters = parameters()?;
    let boot = Path::new("/run/rugix/bakery/work/outputs/boot");
    let config = Path::new("/run/rugix/bakery/work/outputs/config");
    copy_tree(Path::new("/run/rugix/bakery/work/inputs/boot"), boot)?;
    copy_tree(Path::new("/run/rugix/bakery/work/inputs/config"), config)?;
    match p.target {
        Target::RpiTryboot => initialize_tryboot(config)?,
        Target::RpiUboot => initialize_uboot(p.architecture, config)?,
        Target::GenericGrubEfi => initialize_grub(p.architecture, config)?,
        Target::Bsp | Target::Unknown => {}
    }
    if !matches!(p.target, Target::Bsp | Target::Unknown) {
        fs::create_dir_all(config.join(".rugix"))
            .whatever("unable to create bootstrap directory")?;
        File::create(config.join(".rugix/bootstrap"))
            .whatever("unable to create bootstrap marker")?;
    }
    clamp_tree(boot, p.source_date_epoch)?;
    clamp_tree(config, p.source_date_epoch)
}

/// Patch an initialized boot tree with its resolved root partition reference.
pub fn prepare_boot() -> BakeryResult<()> {
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Parameters {
        target: Target,
        source_date_epoch: u64,
    }
    let p: Parameters = parameters()?;
    let boot = Path::new("/run/rugix/bakery/work/outputs/boot");
    copy_tree(Path::new("/run/rugix/bakery/work/inputs/boot"), boot)?;
    let table = PartitionTable::read("/run/rugix/bakery/work/inputs/image")
        .whatever("unable to read boot partition table")?;
    if matches!(
        p.target,
        Target::RpiTryboot | Target::RpiUboot | Target::GenericGrubEfi
    ) {
        let number = if table.is_mbr() { 5 } else { 4 };
        let root = table
            .partitions
            .iter()
            .find(|partition| partition.number == number)
            .ok_or_else(|| reportify::whatever!("boot flow requires root partition {number}"))?;
        let partuuid = match table.disk_id {
            DiskId::Mbr(id) => format!("{:08X}-{number:02}", id.into_raw()),
            DiskId::Gpt(_) => root
                .gpt_id
                .ok_or_else(|| reportify::whatever!("missing root partition GUID"))?
                .to_hex_str(ascii_numbers::Case::Lower)
                .to_string(),
            _ => bail!("unsupported partition table"),
        };
        if matches!(p.target, Target::GenericGrubEfi) {
            rugix_common::grub_patch_env(boot, partuuid)
                .whatever("unable to patch Grub boot environment")?;
        } else {
            rugix_common::rpi_patch_boot(boot, format!("PARTUUID={partuuid}"))
                .whatever("unable to patch Raspberry Pi boot configuration")?;
        }
    }
    clamp_tree(boot, p.source_date_epoch)
}

/// Populate a disk skeleton from numbered filesystem artifacts without loop devices.
pub fn assemble_disk() -> BakeryResult<()> {
    let source = Path::new("/run/rugix/bakery/work/inputs/image");
    let target = Path::new("/run/rugix/bakery/work/outputs/system.img");
    run(command("cp")
        .args(["--reflink=auto", "--sparse=always", "--"])
        .arg(source)
        .arg(target))?;
    let table = PartitionTable::read(source).whatever("unable to read disk skeleton")?;
    let mut disk = File::options()
        .write(true)
        .open(target)
        .whatever("unable to open disk image")?;
    for entry in fs::read_dir("/run/rugix/bakery/work/inputs/partitions")
        .whatever("unable to read partition images")?
    {
        let entry = entry.whatever("unable to read partition image")?;
        let number: u8 = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse().ok())
            .ok_or_else(|| reportify::whatever!("invalid partition image name"))?;
        let partition = table
            .partitions
            .iter()
            .find(|partition| partition.number == number)
            .ok_or_else(|| reportify::whatever!("unknown partition image {number}"))?;
        if partition.ty.is_extended() {
            bail!("cannot populate an extended partition container");
        }
        let mut image = File::open(entry.path()).whatever("unable to open filesystem image")?;
        if image
            .metadata()
            .whatever("unable to inspect filesystem image")?
            .len()
            > table.blocks_to_bytes(partition.size).into_raw()
        {
            bail!("filesystem exceeds partition {number}");
        }
        disk.seek(SeekFrom::Start(
            table.blocks_to_bytes(partition.start).into_raw(),
        ))
        .whatever("unable to seek to partition")?;
        std::io::copy(&mut image, &mut disk).whatever("unable to populate disk partition")?;
    }
    Ok(())
}

pub fn archive_tree() -> BakeryResult<()> {
    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct Parameters {
        source_date_epoch: i64,
    }
    let p: Parameters = parameters()?;
    crate::oven::archive::create(
        Path::new("/run/rugix/bakery/work/inputs/tree"),
        Path::new("/run/rugix/bakery/work/outputs/tree.tar"),
        Some(p.source_date_epoch),
    )
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct BootParameters {
    target: Target,
    architecture: Architecture,
    source_date_epoch: u64,
}

/// Generated boot metadata must have stable identities before downstream caching.
fn clamp_tree(root: &Path, epoch: u64) -> BakeryResult<()> {
    let timestamp = UNIX_EPOCH
        .checked_add(Duration::from_secs(epoch))
        .ok_or_else(|| reportify::whatever!("source date is out of range"))?;
    let mut pending: Vec<PathBuf> = vec![root.to_owned()];
    while let Some(path) = pending.pop() {
        let metadata = fs::symlink_metadata(&path).whatever("unable to inspect boot artifact")?;
        if metadata.is_dir() {
            for entry in fs::read_dir(&path).whatever("unable to read boot artifact")? {
                pending.push(entry.whatever("unable to read boot entry")?.path());
            }
        }
        if metadata
            .modified()
            .whatever("unable to inspect boot timestamp")?
            > timestamp
        {
            run(command("touch")
                .args(["--no-dereference", "--date"])
                .arg(format!("@{epoch}"))
                .arg("--")
                .arg(path))?;
        }
    }
    Ok(())
}
