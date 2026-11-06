//! Filesystem-independent dm-verity integrity images.

use std::fs;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use reportify::{bail, ResultExt};
use serde::Deserialize;
use uuid::Uuid;

use super::{command, file_digest, output, parameters, run, sparse_file};
use crate::BakeryResult;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Verity {
    data_block_size: u64,
    hash_block_size: u64,
    hash_algorithm: String,
    salt: String,
    size: u64,
}

pub fn verity() -> BakeryResult<()> {
    let p: Verity = parameters()?;
    for size in [p.data_block_size, p.hash_block_size] {
        if size < 512 || !size.is_power_of_two() {
            bail!("verity block sizes must be powers of two of at least 512");
        }
    }
    let data_size = fs::metadata("/run/rugix/bakery/work/inputs/image")
        .whatever("unable to inspect verity input")?
        .len();
    if data_size == 0 || data_size % p.data_block_size != 0 {
        bail!("verity data must contain complete data blocks");
    }
    let hash = output(
        command("openssl")
            .args(["dgst", &format!("-{}", p.hash_algorithm), "-binary"])
            .stdin(std::process::Stdio::null()),
    )?;
    let digest_slot = hash.stdout.len().next_power_of_two() as u64;
    let per_block = p.hash_block_size / digest_slot;
    if per_block < 2 {
        bail!("hash blocks must fit at least two digests");
    }
    let mut blocks = data_size / p.data_block_size;
    let mut hash_blocks = 0u64;
    while blocks > 1 {
        blocks = blocks.div_ceil(per_block);
        hash_blocks += blocks;
    }
    let required = (hash_blocks + 1)
        .checked_mul(p.hash_block_size)
        .ok_or_else(|| reportify::whatever!("verity size overflow"))?;
    let size = if p.size == 0 {
        required.div_ceil(1048576) * 1048576
    } else {
        p.size
    };
    if size < required {
        bail!("configured verity image is too small");
    }
    sparse_file("/run/rugix/bakery/work/outputs/verity.img", size)?;
    let digest = hex::encode(file_digest(Path::new(
        "/run/rugix/bakery/work/inputs/image",
    ))?);
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, digest.as_bytes());
    run(command("veritysetup").arg("format").args([
        format!("--uuid={uuid}"),
        format!(
            "--salt={}",
            if p.salt.is_empty() { &digest } else { &p.salt }
        ),
        format!("--hash={}", p.hash_algorithm),
        format!("--data-block-size={}", p.data_block_size),
        format!("--hash-block-size={}", p.hash_block_size),
        "--root-hash-file=/run/rugix/bakery/work/outputs/root.hash".into(),
        "/run/rugix/bakery/work/inputs/image".into(),
        "/run/rugix/bakery/work/outputs/verity.img".into(),
    ]))?;
    let root_hash = fs::read_to_string("/run/rugix/bakery/work/outputs/root.hash")
        .whatever("unable to read verity root hash")?;
    run(command("veritysetup").args([
        "verify",
        "/run/rugix/bakery/work/inputs/image",
        "/run/rugix/bakery/work/outputs/verity.img",
        root_hash.trim(),
    ]))
}

/// Append a detached hash tree to an unchanged data image and export its geometry.
pub fn append_verity() -> BakeryResult<()> {
    #[derive(Deserialize)]
    struct Parameters {
        alignment: u64,
    }
    let p: Parameters = parameters()?;
    if p.alignment < 512 || !p.alignment.is_power_of_two() {
        bail!("verity alignment must be a power of two of at least 512 bytes");
    }
    let data = Path::new("/run/rugix/bakery/work/inputs/image");
    let hashes = Path::new("/run/rugix/bakery/work/inputs/hash-tree");
    let image = Path::new("/run/rugix/bakery/work/outputs/image");
    let data_size = fs::metadata(data)
        .whatever("unable to inspect verity data")?
        .len();
    let hash_size = fs::metadata(hashes)
        .whatever("unable to inspect verity hashes")?
        .len();
    if data_size == 0 || hash_size == 0 {
        bail!("verity data and hash tree must not be empty");
    }
    let offset = data_size
        .div_ceil(p.alignment)
        .checked_mul(p.alignment)
        .ok_or_else(|| reportify::whatever!("verity offset overflow"))?;
    if offset
        .checked_add(hash_size)
        .is_none_or(|size| size > i64::MAX as u64)
    {
        bail!("combined verity image exceeds the supported file size");
    }
    run(command("cp")
        .args(["--reflink=auto", "--sparse=always", "--"])
        .arg(data)
        .arg(image))?;
    let mut destination = fs::OpenOptions::new()
        .write(true)
        .open(image)
        .whatever("unable to open combined verity image")?;
    destination
        .seek(SeekFrom::Start(offset))
        .whatever("unable to seek to verity hash area")?;
    let mut source = fs::File::open(hashes).whatever("unable to open verity hash tree")?;
    std::io::copy(&mut source, &mut destination).whatever("unable to append verity hash tree")?;
    fs::write(
        "/run/rugix/bakery/work/outputs/hash-offset",
        format!("{offset}\n"),
    )
    .whatever("unable to write verity offset")?;
    fs::write(
        "/run/rugix/bakery/work/outputs/data-size",
        format!("{data_size}\n"),
    )
    .whatever("unable to write verity data size")
}
