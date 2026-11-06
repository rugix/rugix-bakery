//! Assemble disk images from Rugix layouts and numbered partition artifacts.
//!
//! The geometry is independent of filesystem creation. GPT serialization uses
//! Rugix's partition table writer; image assembly needs no loop devices.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use byte_calc::NumBytes;
use reportify::{bail, ResultExt};
use rugix_common::disk::gpt::{gpt_types, Guid, GPT_DEFAULT_TABLE_LENGTH, GPT_TABLE_BLOCKS};
use rugix_common::disk::{DiskId, NumBlocks, Partition, PartitionTable, DEFAULT_BLOCK_SIZE};
use rugix_fs::{Copier, File};
use sha2::{Digest, Sha256};

use crate::config::images::{ImageLayout, PartitionTableType};
use crate::BakeryResult;

/// Execute `image-gpt` with the mounted rule inputs and parameters.
pub fn execute() -> BakeryResult<()> {
    #[derive(serde::Deserialize)]
    struct Parameters {
        size: u64,
    }
    let p: Parameters = super::parameters()?;
    image_gpt(
        Path::new("/run/rugix/bakery/work/inputs/layout"),
        Path::new("/run/rugix/bakery/work/inputs/partitions"),
        Path::new("/run/rugix/bakery/work/outputs/image.gpt"),
        (p.size != 0).then_some(NumBytes::new(p.size)),
    )
}

/// Assemble a GPT image from a JSON or TOML layout and numbered input files.
#[tracing::instrument(skip_all, fields(?layout_path, ?image_path))]
pub fn image_gpt(
    layout_path: &Path,
    artifacts_path: &Path,
    image_path: &Path,
    size: Option<NumBytes>,
) -> BakeryResult<()> {
    let layout: ImageLayout = crate::config::load_config_or_json(layout_path)?;
    if layout
        .raw_blobs
        .as_ref()
        .is_some_and(|blobs| !blobs.is_empty())
    {
        bail!("image-gpt does not accept raw blobs; use image-layout and assemble-disk");
    }
    let artifacts = read_artifacts(artifacts_path)?;
    let table = partition_table(&layout, &artifacts, size)?;
    for number in artifacts.keys() {
        if !table
            .partitions
            .iter()
            .any(|partition| partition.number == *number)
        {
            bail!("unknown partition artifact {number}");
        }
    }

    // A fresh destination also prevents overwriting input artifacts or their hard links.
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(image_path)
        .whatever("unable to create disk image")?
        .set_len(table.size().into_raw())
        .whatever("unable to size disk image")?;
    table.write(image_path).whatever("unable to write GPT")?;
    let mut image = File::open_write(image_path).whatever("unable to open disk image")?;
    let mut copier = Copier::new();
    for partition in &table.partitions {
        let Some(length) = artifacts.get(&partition.number) else {
            continue;
        };
        let source = artifacts_path.join(partition.number.to_string());
        let mut source = File::open_read(&source).whatever("unable to open partition artifact")?;
        copier
            .copy_file_range(
                &mut source,
                NumBytes::new(0),
                &mut image,
                NumBytes::new(table.blocks_to_bytes(partition.start).into_raw()),
                NumBytes::new(*length),
            )
            .whatever_with(|_| format!("unable to copy partition {}", partition.number))?;
    }
    Ok(())
}

/// Inspect artifacts without following symbolic links or accepting ambiguous numbers.
fn read_artifacts(directory: &Path) -> BakeryResult<BTreeMap<u8, u64>> {
    let mut artifacts = BTreeMap::new();
    for entry in fs::read_dir(directory).whatever("unable to read partition artifacts")? {
        let entry = entry.whatever("unable to read partition artifact")?;
        let name = entry.file_name();
        let number: u8 = name
            .to_str()
            .and_then(|name| name.parse().ok())
            .filter(|number: &u8| *number > 0 && name == number.to_string().as_str())
            .ok_or_else(|| reportify::whatever!("invalid partition artifact name {name:?}"))?;
        let metadata = entry
            .metadata()
            .whatever("unable to inspect partition artifact")?;
        if !metadata.is_file() {
            bail!("partition artifact {number} must be a regular file");
        }
        artifacts.insert(number, metadata.len());
    }
    Ok(artifacts)
}

/// Resolve capacities and alignment, keeping all arithmetic checked before writing.
fn partition_table(
    layout: &ImageLayout,
    artifacts: &BTreeMap<u8, u64>,
    size: Option<NumBytes>,
) -> BakeryResult<PartitionTable> {
    if layout.ty != Some(PartitionTableType::Gpt) {
        bail!("image-gpt requires a GPT layout");
    }
    let explicit_blocks = size
        .map(|size| -> BakeryResult<u64> {
            if size.raw % DEFAULT_BLOCK_SIZE != 0 {
                bail!("disk size must be a multiple of {DEFAULT_BLOCK_SIZE} bytes");
            }
            let blocks = size.raw / DEFAULT_BLOCK_SIZE;
            if blocks < 2 * GPT_TABLE_BLOCKS.into_raw() + 1 {
                bail!("disk size is too small for GPT metadata");
            }
            Ok(blocks)
        })
        .transpose()?;
    let mut table = PartitionTable::new(
        DiskId::Gpt(Guid::from_bytes([0; 16])),
        NumBlocks::from_raw(0),
    );
    let mut next_start = ALIGNMENT;
    let specifications = layout.partitions.as_deref().unwrap_or_default();
    if specifications.len() as u64 > GPT_DEFAULT_TABLE_LENGTH {
        bail!("too many GPT partitions");
    }
    for (index, specification) in specifications.iter().enumerate() {
        let number = u8::try_from(index + 1).whatever("too many GPT partitions")?;
        if specification.root.is_some()
            || specification.filesystem.is_some()
            || specification.bootable.is_some()
        {
            bail!("image-gpt accepts partition images; root, filesystem, and bootable belong to image-layout");
        }
        let capacity = specification
            .size
            .as_ref()
            .map(|size| size.raw)
            .or_else(|| artifacts.get(&number).copied())
            .ok_or_else(|| reportify::whatever!("partition {number} needs a size or an artifact"))?
            .div_ceil(DEFAULT_BLOCK_SIZE);
        if capacity == 0 {
            bail!("partition {number} must have a positive capacity");
        }
        let end = next_start
            .checked_add(capacity)
            .ok_or_else(|| reportify::whatever!("partition geometry overflow"))?;
        let capacity_bytes = capacity
            .checked_mul(DEFAULT_BLOCK_SIZE)
            .ok_or_else(|| reportify::whatever!("partition capacity overflow"))?;
        if artifacts
            .get(&number)
            .is_some_and(|length| *length > capacity_bytes)
        {
            bail!("payload exceeds partition {number}");
        }
        let ty = specification
            .ty
            .as_ref()
            .map(|ty| ty.0)
            .unwrap_or(gpt_types::LINUX);
        if !matches!(ty, rugix_common::disk::PartitionType::Gpt(guid) if !guid.is_zero()) {
            bail!("partition {number} requires a nonzero GPT type GUID");
        }
        table.partitions.push(Partition {
            number,
            start: NumBlocks::from_raw(next_start),
            size: NumBlocks::from_raw(capacity),
            ty,
            name: specification.name.as_ref().map(ToString::to_string),
            gpt_id: None,
            gpt_attrs: None,
            bootable: false,
        });
        next_start = align(end)?;
    }
    let disk_blocks = match explicit_blocks {
        Some(blocks) => blocks,
        None => {
            let end = table
                .partitions
                .last()
                .map(|partition| partition.end().into_raw())
                .unwrap_or(ALIGNMENT);
            align(
                end.checked_add(GPT_TABLE_BLOCKS.into_raw())
                    .ok_or_else(|| reportify::whatever!("disk geometry overflow"))?,
            )?
        }
    };
    if disk_blocks
        .checked_mul(DEFAULT_BLOCK_SIZE)
        .is_none_or(|bytes| bytes > i64::MAX as u64)
    {
        bail!("disk size exceeds the supported file size");
    }
    let mut seed = Sha256::new();
    seed.update(b"rugix-gpt-layout-v1\0");
    seed.update(serde_json::to_vec(layout).whatever("unable to encode disk layout")?);
    seed.update(disk_blocks.to_le_bytes());
    for partition in &table.partitions {
        seed.update([partition.number]);
        seed.update(partition.start.into_raw().to_le_bytes());
        seed.update(partition.size.into_raw().to_le_bytes());
    }
    let disk_id = derived_guid(&seed.finalize(), 0);
    table.disk_id = DiskId::Gpt(disk_id);
    for partition in &mut table.partitions {
        partition.gpt_id = Some(derived_guid(
            disk_id.to_string().as_bytes(),
            partition.number,
        ));
    }
    table.disk_size = NumBlocks::from_raw(disk_blocks);
    table.validate().whatever("invalid disk layout")?;
    Ok(table)
}

/// Partition starts and automatically sized disks are aligned to one MiB.
const ALIGNMENT: u64 = (1024 * 1024) / DEFAULT_BLOCK_SIZE;

/// Round upward without allowing untrusted sizes to overflow.
fn align(blocks: u64) -> BakeryResult<u64> {
    blocks
        .div_ceil(ALIGNMENT)
        .checked_mul(ALIGNMENT)
        .ok_or_else(|| reportify::whatever!("disk alignment overflow"))
}

/// Derive domain-separated GUIDs without consulting the random number generator.
fn derived_guid(seed: &[u8], number: u8) -> Guid {
    let mut hash = Sha256::new();
    hash.update(b"rugix-gpt-guid-v1\0");
    hash.update(seed);
    hash.update([number]);
    let mut bytes: [u8; 16] = hash.finalize()[..16]
        .try_into()
        .expect("SHA-256 has 32 bytes");
    bytes[7] = (bytes[7] & 0x0f) | 0x80;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Guid::from_bytes(bytes)
}
