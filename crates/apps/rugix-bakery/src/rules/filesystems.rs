//! Reproducible userspace filesystem creation from directory artifacts.

use std::collections::HashSet;
use std::fs::{self, File};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use reportify::{bail, ResultExt};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::{digest, epoch_command, file_digest, options, output, parameters, run, sparse_file};
use crate::BakeryResult;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Ext4 {
    size: u64,
    label: String,
    uuid: String,
    block_size: u64,
    reserved_percent: u8,
    mkfs_options: String,
}

pub fn ext4() -> BakeryResult<()> {
    let p: Ext4 = parameters()?;
    let epoch = super::source_date_epoch()?;
    let image = "/run/rugix/bakery/work/outputs/image.ext4";
    if !p.block_size.is_power_of_two() || !(1024..=65536).contains(&p.block_size) {
        bail!("ext4 block-size must be a power of two between 1024 and 65536");
    }
    let mut tar = epoch_command("tar", epoch)?;
    tar.args([
        "--create",
        "--file=-",
        "--format=pax",
        "--sort=name",
        "--numeric-owner",
        "--acls",
        "--xattrs",
        "--xattrs-include=*",
        "--pax-option=exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime",
        "--clamp-mtime",
    ])
    .arg(format!("--mtime=@{}", epoch))
    .args(["--directory=/run/rugix/bakery/work/inputs/tree", "."])
    .stdout(Stdio::piped());
    let mut archive = tar.spawn().whatever("unable to hash filesystem tree")?;
    let hashed = digest(archive.stdout.as_mut().expect("tar stdout is piped"));
    let status = archive.wait().whatever("unable to wait for tree archive")?;
    let hash = hashed?;
    if !status.success() {
        bail!("unable to archive filesystem tree");
    }
    let seed = Uuid::new_v5(
        &Uuid::NAMESPACE_OID,
        format!("rugix-ext4:{}", hex::encode(hash)).as_bytes(),
    );
    let automatic = p.size == 0;
    let mut inodes = 0u64;
    let size = if automatic {
        let mut bytes = 0u64;
        let mut seen = HashSet::new();
        let mut pending = vec![PathBuf::from("/run/rugix/bakery/work/inputs/tree")];
        while let Some(path) = pending.pop() {
            let metadata =
                fs::symlink_metadata(&path).whatever("unable to inspect filesystem input")?;
            if !seen.insert((metadata.dev(), metadata.ino())) {
                continue;
            }
            inodes += 1;
            let length = if metadata.is_dir() {
                p.block_size
            } else {
                metadata.len()
            };
            bytes = bytes
                .checked_add(length.div_ceil(p.block_size) * p.block_size)
                .ok_or_else(|| reportify::whatever!("filesystem size overflow"))?;
            if metadata.is_dir() {
                for entry in fs::read_dir(path).whatever("unable to read filesystem tree")? {
                    pending.push(entry.whatever("unable to read filesystem entry")?.path());
                }
            }
        }
        let provision = bytes
            .checked_add(inodes * p.block_size * 4 + 32 * 1024 * 1024)
            .and_then(|size| size.checked_mul(5))
            .ok_or_else(|| reportify::whatever!("filesystem size overflow"))?
            / 4;
        provision.div_ceil(1048576) * 1048576
    } else {
        p.size
    };
    sparse_file(image, size)?;
    let mut mkfs = epoch_command("mkfs.ext4", epoch)?;
    mkfs.args(["-q", "-F", "-b"])
        .arg(p.block_size.to_string())
        .arg("-m")
        .arg(p.reserved_percent.to_string())
        .arg("-E")
        .arg(format!(
            "lazy_itable_init=0,lazy_journal_init=0,hash_seed={seed}"
        ))
        .arg("-U")
        .arg(if p.uuid.is_empty() {
            seed.to_string()
        } else {
            p.uuid
        });
    if automatic {
        mkfs.arg("-N")
            .arg((inodes + 1024.max(inodes / 10)).to_string());
    }
    if !p.label.is_empty() {
        mkfs.args(["-L", &p.label]);
    }
    run(mkfs.args(options(&p.mkfs_options)?).args([
        "-d",
        "/run/rugix/bakery/work/inputs/tree",
        image,
    ]))?;
    if automatic {
        run(epoch_command("resize2fs", epoch)?.args(["-M", image]))?;
        let metadata = output(epoch_command("dumpe2fs", epoch)?.args(["-h", image]))?;
        let text = String::from_utf8(metadata.stdout).whatever("invalid ext4 metadata")?;
        let field = |key: &str| -> BakeryResult<u64> {
            text.lines()
                .filter_map(|line| line.split_once(':'))
                .find(|(name, _)| *name == key)
                .ok_or_else(|| reportify::whatever!("missing ext4 metadata {key}"))?
                .1
                .trim()
                .parse::<u64>()
                .whatever("invalid ext4 geometry")
        };
        File::options()
            .write(true)
            .open(image)
            .whatever("unable to open ext4 image")?
            .set_len(field("Block count")? * field("Block size")?)
            .whatever("unable to truncate ext4 image")?;
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Fat {
    size: u64,
    label: String,
    volume_id: String,
    sectors_per_cluster: u64,
    mkfs_options: String,
}

pub fn fat() -> BakeryResult<()> {
    let p: Fat = parameters()?;
    let epoch = super::source_date_epoch()?;
    let image = "/run/rugix/bakery/work/outputs/image.fat";
    let root = Path::new("/run/rugix/bakery/work/inputs/tree");
    let mut identity = Sha256::new();
    let mut tree = Vec::new();
    let mut file_sizes = Vec::new();
    let mut directory_slots = Vec::new();
    let canonical_root = fs::canonicalize(root).whatever("unable to resolve FAT tree")?;
    let mut pending = vec![(root.to_path_buf(), vec![canonical_root.clone()])];
    while let Some((directory, ancestors)) = pending.pop() {
        let mut entries = fs::read_dir(&directory)
            .whatever("unable to read FAT tree")?
            .collect::<Result<Vec<_>, _>>()
            .whatever("unable to read FAT entry")?;
        entries.sort_by_key(|entry| entry.file_name());
        let mut files = Vec::new();
        let mut directories = Vec::new();
        let mut slots = 2u64;
        for entry in entries {
            let resolved =
                fs::canonicalize(entry.path()).whatever("unable to resolve FAT entry")?;
            if !resolved.starts_with(&canonical_root) {
                bail!("FAT tree symlinks must stay within the input tree");
            }
            let metadata = fs::metadata(&resolved).whatever("unable to inspect FAT entry")?;
            if !metadata.is_dir() && !metadata.is_file() {
                bail!("FAT trees support regular files and directories");
            }
            let relative = entry
                .path()
                .strip_prefix(root)
                .expect("entry is within FAT tree")
                .to_path_buf();
            identity.update(relative.as_os_str().as_encoded_bytes());
            identity.update([0, u8::from(metadata.is_dir())]);
            if metadata.is_file() {
                file_sizes.push(metadata.len());
                identity.update(file_digest(&entry.path())?);
                files.push(entry.path());
            } else {
                if ancestors.contains(&resolved) {
                    bail!("FAT tree contains a directory symlink cycle");
                }
                let mut ancestors = ancestors.clone();
                ancestors.push(resolved);
                directories.push((entry.path(), ancestors));
            }
            let name = entry.file_name();
            let name = name
                .to_str()
                .ok_or_else(|| reportify::whatever!("FAT filenames must be UTF-8"))?;
            slots += 1 + (name.encode_utf16().count() as u64).div_ceil(13);
        }
        directory_slots.push(slots);
        tree.push((
            directory
                .strip_prefix(root)
                .expect("directory is in tree")
                .to_path_buf(),
            files,
        ));
        pending.extend(directories.into_iter().rev());
    }
    let mut sectors = p.sectors_per_cluster;
    if sectors != 0 && (!sectors.is_power_of_two() || sectors > 128) {
        bail!("invalid FAT sectors-per-cluster");
    }
    let size = if p.size == 0 {
        sectors = sectors.max(1);
        let clusters = loop {
            let cluster = sectors * 512;
            let required = file_sizes
                .iter()
                .map(|size| size.div_ceil(cluster))
                .sum::<u64>()
                + directory_slots
                    .iter()
                    .map(|slots| (slots * 32).div_ceil(cluster))
                    .sum::<u64>();
            let clusters = 65525.max(required + required / 100 + 32);
            if clusters <= 0x0ffffff5 {
                break clusters;
            }
            if p.sectors_per_cluster != 0 || sectors >= 128 {
                bail!("tree exceeds FAT32 cluster capacity");
            }
            sectors *= 2;
        };
        let fat_sectors = ((clusters + 2) * 4).div_ceil(512);
        (32 * 512 + 2 * fat_sectors * 512 + clusters * sectors * 512).div_ceil(1048576) * 1048576
    } else {
        p.size
    };
    sparse_file(image, size)?;
    let mut mkfs = epoch_command("mkfs.vfat", epoch)?;
    let volume_id = if p.volume_id.is_empty() {
        hex::encode(identity.finalize())[..8].to_owned()
    } else {
        p.volume_id
    };
    mkfs.args(["--invariant", "-i", &volume_id]);
    if p.size == 0 {
        // Automatic sizing uses FAT32 geometry; fixed images retain mkfs's format selection.
        mkfs.args(["-F", "32"]);
    }
    if !p.label.is_empty() {
        mkfs.args(["-n", &p.label]);
    }
    if sectors != 0 {
        mkfs.arg("-s").arg(sectors.to_string());
    }
    run(mkfs.args(options(&p.mkfs_options)?).arg(image))?;
    for (relative, files) in tree {
        let destination = format!("::/{}", relative.display());
        if !relative.as_os_str().is_empty() {
            run(epoch_command("mmd", epoch)?.args(["-i", image, &destination]))?;
        }
        if !files.is_empty() {
            run(epoch_command("mcopy", epoch)?
                .args(["-i", image])
                .args(files)
                .arg(format!("{}/", destination.trim_end_matches('/'))))?;
        }
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Squashfs {
    compression: String,
    block_size: u64,
    mkfs_options: String,
}

pub fn squashfs() -> BakeryResult<()> {
    let p: Squashfs = parameters()?;
    let epoch = super::source_date_epoch()?;
    run(epoch_command("mksquashfs", epoch)?
        .args([
            "/run/rugix/bakery/work/inputs/tree",
            "/run/rugix/bakery/work/outputs/image.squashfs",
            "-noappend",
            "-processors",
            "1",
            "-comp",
            &p.compression,
            "-b",
        ])
        .arg(p.block_size.to_string())
        .args(options(&p.mkfs_options)?))
}
