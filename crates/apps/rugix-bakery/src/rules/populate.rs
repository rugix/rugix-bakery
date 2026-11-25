//! Populate the numbered partition images of an unpopulated disk layout.

use std::fs;
use std::path::{Path, PathBuf};

use crate::config::images::{Filesystem, ImageLayout, SquashfsOptions};
use crate::BakeryResult;
use reportify::{bail, whatever, ResultExt};
use rugix_common::disk::PartitionTable;
use sidex::types::Str;
use tempfile::TempDir;

use super::filesystems::{build_ext4, build_fat, build_squashfs, Ext4, Fat, Squashfs};
use super::layout::contained_path;
use super::source_date_epoch;

pub fn image_populate_partitions() -> BakeryResult<()> {
    let inputs = Path::new("/run/rugix/bakery/work/inputs");
    let outputs = Path::new("/run/rugix/bakery/work/outputs/partitions");
    let epoch = source_date_epoch()?;

    let layout: ImageLayout = crate::config::load_config_or_json(&inputs.join("layout"))?;
    let table = PartitionTable::read(&inputs.join("image"))
        .whatever("unable to read resolved disk geometry")?;
    populate_partitions(&layout, &table, &inputs.join("roots"), outputs, epoch)
}

/// Build one numbered image per filesystem partition of `layout`.
///
/// `roots` is the prepared filesystem-tree directory, `outputs` is created if
/// missing. Partitions without a filesystem are skipped; a filesystem partition
/// with zero capacity is an error.
fn populate_partitions(
    layout: &ImageLayout,
    table: &PartitionTable,
    roots: &Path,
    outputs: &Path,
    epoch: u64,
) -> BakeryResult<()> {
    fs::create_dir_all(outputs).whatever("unable to create partition outputs")?;

    let specifications: Vec<_> = layout.partitions.iter().flatten().collect();
    if specifications.len() != table.partitions.len() {
        bail!(
            "layout declares {} partitions but the disk table has {}",
            specifications.len(),
            table.partitions.len()
        );
    }

    for (specification, partition) in specifications.iter().zip(&table.partitions) {
        let Some(filesystem) = &specification.filesystem else {
            // Raw firmware regions, uninitialized, or extended container
            // partitions: nothing to populate for `core/assemble-disk`.
            continue;
        };
        let capacity = table.blocks_to_bytes(partition.size).into_raw();
        if capacity == 0 {
            bail!("partition {} has no capacity", partition.number);
        }
        let image = outputs.join(partition.number.to_string());
        let image = image
            .to_str()
            .ok_or_else(|| whatever!("partition image path is not UTF-8"))?;
        let (tree, _staging) = partition_tree(roots, specification.root.as_deref())?;
        match filesystem {
            Filesystem::Ext4(options) => {
                // The partition image is built from the raw tree: `clamp-mtime`
                // is an archive concern (see `core/archive-tree`), and
                // `build_ext4` already reproduces the image via the source epoch.
                build_ext4(
                    &tree,
                    image,
                    &Ext4 {
                        size: capacity,
                        label: String::new(),
                        uuid: String::new(),
                        block_size: 4096,
                        reserved_percent: 0,
                        mkfs_options: join_options(options.additional_options.as_deref())?,
                    },
                    epoch,
                )?;
            }
            Filesystem::Fat32 => {
                build_fat(
                    &tree,
                    image,
                    &Fat {
                        size: capacity,
                        minimum_size: 0,
                        free_percent: 0,
                        size_alignment: 0,
                        label: String::new(),
                        volume_id: String::new(),
                        sectors_per_cluster: 0,
                        mkfs_options: String::new(),
                    },
                    epoch,
                )?;
            }
            Filesystem::Squashfs(options) => {
                build_squashfs(
                    &tree,
                    image,
                    &Squashfs {
                        compression: "gzip".into(),
                        block_size: 131072,
                        mkfs_options: squashfs_mkfs_options(options)?,
                    },
                    epoch,
                )?;
            }
        }
    }
    Ok(())
}

/// Resolve a partition root to a tree.
///
/// * no root — an empty staged tree (initializes an empty filesystem, as the oven
///   pipeline does for filesystems without a root),
/// * root — the `roots` input subtree, containment-checked the same way
///   `core/image-layout` sized it.
///
/// The returned `TempDir` (when staging) must be kept alive until the
/// filesystem image has been built.
fn partition_tree(roots: &Path, root: Option<&str>) -> BakeryResult<(PathBuf, Option<TempDir>)> {
    let Some(root) = root else {
        let staging = tempfile::Builder::new()
            .prefix("populate-empty-")
            .tempdir()
            .whatever("unable to stage an empty tree")?;
        let tree = staging.path().join("tree");
        fs::create_dir_all(&tree).whatever("unable to create an empty tree")?;
        return Ok((tree, Some(staging)));
    };
    Ok((contained_path(roots, root)?, None))
}

/// Assemble squashfs formatter arguments, appending the no-compression flags.
fn squashfs_mkfs_options(options: &SquashfsOptions) -> BakeryResult<String> {
    let mut additional = options
        .additional_options
        .as_deref()
        .unwrap_or_default()
        .to_vec();
    if options.no_compression.unwrap_or(false) {
        additional.extend(["-noI", "-noD", "-noF", "-noX"].map(Str::from));
    }
    join_options(Some(&additional))
}

/// Shell-join formatter arguments the way rule parameters carry them.
fn join_options(options: Option<&[Str]>) -> BakeryResult<String> {
    shlex::try_join(
        options
            .unwrap_or_default()
            .iter()
            .map(|value| value.as_str()),
    )
    .whatever("formatter arguments contain NUL")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::images::{ImagePartition, PartitionTableType};
    use rugix_common::disk::gpt::gpt_types;
    use rugix_common::disk::{DiskId, NumBlocks, Partition};

    /// A layout partition with an optional filesystem.
    fn partition_spec(filesystem: Option<Filesystem>) -> ImagePartition {
        ImagePartition {
            name: None,
            size: None,
            filesystem,
            root: None,
            ty: None,
            bootable: None,
        }
    }

    /// Build a GPT layout from the given partition specs.
    fn layout_with(specs: Vec<ImagePartition>) -> ImageLayout {
        ImageLayout {
            ty: Some(PartitionTableType::Gpt),
            raw_blobs: None,
            partitions: Some(specs.into()),
        }
    }

    /// An empty GPT table with `count` GPT linux partitions, one per number starting at
    /// 1.
    fn test_table(count: u8) -> PartitionTable {
        let mut table = PartitionTable::new(DiskId::random_gpt(), NumBlocks::from_raw(1 << 16));
        table.partitions = (1..=count)
            .map(|number| Partition {
                number,
                start: NumBlocks::from_raw(2048 * u64::from(number)),
                size: NumBlocks::from_raw(2048),
                ty: gpt_types::LINUX,
                name: None,
                gpt_id: None,
                gpt_attrs: None,
                bootable: false,
            })
            .collect();
        table
    }

    /// Run the orchestration for a layout/table pair, creating `roots` and
    /// returning the outputs directory.
    fn run_for_test(
        temp: &tempfile::TempDir,
        layout: &ImageLayout,
        table: &PartitionTable,
    ) -> BakeryResult<std::path::PathBuf> {
        let roots = temp.path().join("roots");
        std::fs::create_dir_all(&roots).unwrap();
        let outputs = temp.path().join("outputs");
        populate_partitions(layout, table, &roots, &outputs, 0)?;
        Ok(outputs)
    }

    #[test]
    fn layout_partition_count_must_match_the_table() {
        let temp = tempfile::tempdir().unwrap();
        let layout = layout_with(vec![partition_spec(None)]);
        let table = test_table(2);
        let roots = temp.path().join("roots");
        let outputs = temp.path().join("outputs");
        assert!(populate_partitions(&layout, &table, &roots, &outputs, 0).is_err());
    }

    #[test]
    fn empty_layout_creates_output_directory_with_no_files() {
        let temp = tempfile::tempdir().unwrap();
        let layout = ImageLayout {
            ty: Some(PartitionTableType::Gpt),
            raw_blobs: None,
            partitions: None,
        };
        let table = test_table(0);
        let outputs = run_for_test(&temp, &layout, &table).unwrap();
        assert!(outputs.is_dir());
        assert!(std::fs::read_dir(&outputs).unwrap().next().is_none());
    }

    #[test]
    fn partitions_without_a_filesystem_are_skipped() {
        let temp = tempfile::tempdir().unwrap();
        let layout = layout_with(vec![partition_spec(None)]);
        let table = test_table(1);
        let outputs = run_for_test(&temp, &layout, &table).unwrap();
        assert!(std::fs::read_dir(&outputs).unwrap().next().is_none());
    }

    #[test]
    fn filesystem_partition_with_zero_capacity_bails() {
        let temp = tempfile::tempdir().unwrap();
        let layout = layout_with(vec![partition_spec(Some(Filesystem::Fat32))]);
        let mut table = test_table(1);
        table.partitions[0].size = NumBlocks::from_raw(0);
        let roots = temp.path().join("roots");
        let outputs = temp.path().join("outputs");
        assert!(populate_partitions(&layout, &table, &roots, &outputs, 0).is_err());
    }

    #[test]
    fn partition_tree_staging_and_containment() {
        let temp = tempfile::tempdir().unwrap();
        let roots = temp.path().join("roots");
        std::fs::create_dir_all(roots.join("system")).unwrap();
        // A contained root resolves directly inside the prepared tree.
        let (tree, staging) = partition_tree(&roots, Some("system")).unwrap();
        assert!(staging.is_none());
        assert!(tree.ends_with("system"));
        assert!(tree.exists());
        // A partition without a root is built from a staged empty tree.
        let (tree, _staging) = partition_tree(&roots, None).unwrap();
        assert!(tree.exists());
        // Escaping roots are rejected by the same containment check as image-layout.
        assert!(partition_tree(&roots, Some("../escape")).is_err());
    }

    /// The no-compression flags are appended after the declared options.
    #[test]
    fn squashfs_no_compression_flags() {
        let options = SquashfsOptions {
            additional_options: Some(["-no-exports".into()].into()),
            no_compression: Some(true),
        };
        assert_eq!(
            squashfs_mkfs_options(&options).unwrap(),
            "-no-exports -noI -noD -noF -noX"
        );
    }

    /// Options containing NUL bytes cannot be shell-joined and are rejected.
    #[test]
    fn reject_nul_in_options() {
        let options = SquashfsOptions {
            additional_options: Some(["bad\0arg".into()].into()),
            no_compression: None,
        };
        assert!(squashfs_mkfs_options(&options).is_err());
    }
}
