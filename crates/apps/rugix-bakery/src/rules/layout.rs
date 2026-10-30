//! Resolve Bakery image layouts, including MBR extended partitions and firmware regions.

use crate::config::images::{ImageLayout, RawBlob};
use crate::BakeryResult;
use reportify::{bail, whatever, ResultExt};
use rugix_common::disk::gpt::gpt_types;
use rugix_common::disk::mbr::mbr_types;
use rugix_common::disk::{DiskId, NumBlocks, Partition, PartitionTable, PartitionTableType};
use rugix_common::utils::units::NumBytes;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Seek;
use std::path::{Path, PathBuf};

/// We are calculating everything with a portable block size of 512 bytes.
const BLOCK_SIZE: NumBytes = NumBytes::from_raw(512);

/// We align everything to 2048 blocks, i.e., 1MiB.
const ALIGNMENT: NumBlocks = NumBlocks::from_raw(2048);

/// Convert number of bytes to number of blocks.
fn bytes_to_blocks(bytes: NumBytes) -> NumBlocks {
    NumBlocks::from_raw(bytes.into_raw().div_ceil(BLOCK_SIZE.into_raw()))
}

/// Compute the partition table for an image based on the provided layout.
pub fn compute_partition_table(
    layout: &ImageLayout,
    roots_dir: &Path,
    layer_path: &Path,
) -> BakeryResult<PartitionTable> {
    let table_type = layout
        .ty
        .map(|ty| match ty {
            crate::config::images::PartitionTableType::Mbr => PartitionTableType::Mbr,
            crate::config::images::PartitionTableType::Gpt => PartitionTableType::Gpt,
        })
        .unwrap_or(PartitionTableType::Mbr);
    let mut partitions = Vec::new();
    // Start partitions after any raw blobs so they don't overlap.
    let mut next_usable = ALIGNMENT;
    if let Some(raw_blobs) = &layout.raw_blobs {
        let mut regions = Vec::new();
        for blob in raw_blobs {
            let path = contained_path(layer_path, &blob.file)?;
            let actual = fs::metadata(&path)
                .whatever("unable to inspect raw blob")?
                .len();
            let region_size = blob.size.as_ref().map_or(actual, |reserved| reserved.raw);
            if region_size < actual {
                bail!("reserved raw blob region is smaller than its contents");
            }
            let blob_end = blob
                .offset
                .raw
                .checked_add(region_size)
                .ok_or_else(|| whatever!("raw blob offset overflow"))?;
            let data_end = blob
                .offset
                .raw
                .checked_add(actual)
                .ok_or_else(|| whatever!("raw blob offset overflow"))?;
            let metadata_end = if table_type.is_gpt() { 34 * 512 } else { 512 };
            if actual > 0
                && blob.offset.raw < metadata_end
                && !(table_type.is_mbr() && data_end <= 440)
            {
                bail!("raw firmware overlaps partition table metadata");
            }
            if regions
                .iter()
                .any(|(start, end)| blob.offset.raw < *end && *start < blob_end)
            {
                bail!("raw firmware reservations overlap");
            }
            regions.push((blob.offset.raw, blob_end));
            let blob_end_blocks = bytes_to_blocks(NumBytes::from_raw(blob_end));
            let aligned = blob_end_blocks.ceil_align_to(ALIGNMENT);
            if aligned > next_usable {
                next_usable = aligned;
            }
        }
    }
    let mut next_number = 1u8;
    let mut in_extended = false;
    if let Some(layout_partitions) = &layout.partitions {
        for partition in layout_partitions {
            let number = next_number;
            next_number = next_number
                .checked_add(1)
                .ok_or_else(|| whatever!("too many image partitions"))?;
            if table_type.is_mbr() && number > 4 && !in_extended {
                bail!("invalid number of primary partitions in MBR");
            }
            // Leave space for the EBR, if we are creating a logical MBR partition.
            if in_extended {
                next_usable = (next_usable + NumBlocks::ONE).ceil_align_to(ALIGNMENT);
            }
            let partition_type = partition.ty.unwrap_or(match table_type {
                PartitionTableType::Mbr => mbr_types::LINUX.into(),
                PartitionTableType::Gpt => gpt_types::LINUX.into(),
            });
            if layout
                .ty
                .unwrap_or(crate::config::images::PartitionTableType::Mbr)
                != partition_type.table_type()
            {
                bail!("partition type `{partition_type}` does not match table type `{table_type}`",)
            }
            let start = next_usable;
            if partition_type.is_extended() {
                if in_extended {
                    bail!("nested extended partitions are not allowed")
                }
                partitions.push(Partition {
                    number,
                    start,
                    // We fix this later once we know the size of the extended part.
                    size: 0.into(),
                    ty: partition_type.0,
                    name: None,
                    gpt_id: None,
                    gpt_attrs: None,
                    bootable: partition.bootable.unwrap_or(false),
                });
                in_extended = true;
                next_number = 5;
                // Space for the EBR is automatically added prior to the next partition.
            } else {
                let size = match &partition.size {
                    Some(size) => bytes_to_blocks(size.raw.into()),
                    None => {
                        let Some(path) = &partition.root else {
                            bail!("partitions without a fixed size must have a root path");
                        };
                        compute_fs_size(contained_path(roots_dir, path)?)?
                    }
                };
                partitions.push(Partition {
                    number,
                    start,
                    size,
                    ty: partition_type.0,
                    name: None,
                    gpt_id: None,
                    gpt_attrs: None,
                    bootable: partition.bootable.unwrap_or(false),
                });
                next_usable = (start + size).ceil_align_to(ALIGNMENT);
            }
        }
    }
    // Fix the size of the extended partition, if there is one.
    for partition in partitions.iter_mut() {
        if !partition.ty.is_extended() {
            continue;
        }
        partition.size = (next_usable - partition.start + NumBlocks::ONE).ceil_align_to(ALIGNMENT);
        break;
    }
    let image_size = match partitions.last() {
        Some(last_partition) => {
            (last_partition.start + last_partition.size).ceil_align_to(ALIGNMENT) + ALIGNMENT
        }
        None => ALIGNMENT * 32,
    };
    let mut seed = Sha256::new();
    seed.update(b"rugix-standard-layout-v1");
    seed.update(serde_json::to_vec(layout).whatever("unable to encode image layout")?);
    for partition in &partitions {
        seed.update(partition.start.into_raw().to_le_bytes());
        seed.update(partition.size.into_raw().to_le_bytes());
    }
    let seed = seed.finalize();
    let disk_uuid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, &seed);
    let table_id = match table_type {
        PartitionTableType::Mbr => DiskId::Mbr(rugix_common::disk::mbr::MbrId::new(
            u32::from_le_bytes(seed[..4].try_into().expect("SHA-256 is 32 bytes")) | 1,
        )),
        PartitionTableType::Gpt => DiskId::Gpt(
            rugix_common::disk::gpt::Guid::from_hex_str(&disk_uuid.to_string())
                .whatever("invalid generated disk GUID")?,
        ),
    };
    if table_type.is_gpt() {
        for partition in &mut partitions {
            let id = uuid::Uuid::new_v5(&disk_uuid, &[partition.number]);
            partition.gpt_id = Some(
                rugix_common::disk::gpt::Guid::from_hex_str(&id.to_string())
                    .whatever("invalid generated partition GUID")?,
            );
        }
    }
    let mut table = PartitionTable::new(table_id, image_size);
    table.partitions = partitions;
    table
        .validate()
        .whatever("unable to validate image partitions")?;
    Ok(table)
}

/// Write raw firmware blobs at their specified byte offsets on the disk image.
pub fn write_raw_blobs(
    blobs: &[RawBlob],
    layer_path: &Path,
    image_file: &Path,
) -> BakeryResult<()> {
    for blob in blobs {
        let path = contained_path(layer_path, &blob.file)?;
        let mut source = File::open(path).whatever("unable to open raw firmware")?;
        let mut destination = File::options()
            .write(true)
            .open(image_file)
            .whatever("unable to open disk skeleton")?;
        destination
            .seek(std::io::SeekFrom::Start(blob.offset.raw))
            .whatever("unable to seek to firmware region")?;
        std::io::copy(&mut source, &mut destination).whatever("unable to write raw firmware")?;
    }
    Ok(())
}

/// Compute the required size for a filesystem based on the given root path.
fn compute_fs_size(root: PathBuf) -> BakeryResult<NumBlocks> {
    let mut size = NumBytes::from_raw(0);
    let mut stack = vec![root];
    while let Some(top) = stack.pop() {
        // We do not want to follow symlinks here as we are interested in the size of
        // the symlink and not the size of the symlink's target.
        let metadata = fs::symlink_metadata(&top).whatever("unable to get file metadata")?;
        let length = if metadata.is_dir() {
            4096
        } else {
            metadata.len().div_ceil(4096) * 4096
        };
        size += NumBytes::from_raw(length);
        if metadata.is_dir() {
            for entry in fs::read_dir(&top).whatever("unable to read directory")? {
                stack.push(entry.whatever("unable to read directory entry")?.path());
            }
        }
    }
    size = size.max(NumBytes::from_raw(64 * 1024 * 1024));
    // Add an overhead of 20% for filesystem metadata.
    size += NumBytes::from_raw(size.into_raw().div_ceil(5));
    Ok(bytes_to_blocks(size))
}

/// Layout paths must remain inside their declared artifact, including through symlinks.
fn contained_path(root: &Path, relative: &str) -> BakeryResult<PathBuf> {
    let relative = Path::new(relative);
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        bail!("layout paths must be nonempty relative paths without '..'");
    }
    let root = fs::canonicalize(root).whatever("unable to resolve layout input")?;
    let path = fs::canonicalize(root.join(relative)).whatever("unable to resolve layout source")?;
    if !path.starts_with(root) {
        bail!("layout path escapes its input artifact");
    }
    Ok(path)
}
