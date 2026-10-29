//! Resolve artifact selections and assemble cached, metadata-preserving directory trees.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use reportify::{bail, ResultExt};
use sha2::{Digest, Sha256};
use sidex::types::Str;
use xscript::{run, Run};

use crate::config::rules::ArtifactKind;
use crate::BakeryResult;

use super::cache::artifact_hash;
use super::{Artifact, ArtifactLocation};

/// One execution's content identities and directory assembly store.
pub struct Artifacts {
    trees: PathBuf,
    hashes: HashMap<PathBuf, String>,
}

impl Artifacts {
    pub fn new(cache: &Path) -> BakeryResult<Self> {
        let trees = cache.join("trees");
        fs::create_dir_all(&trees).whatever("unable to create artifact tree cache")?;
        Ok(Self {
            trees: fs::canonicalize(trees).whatever("unable to resolve artifact tree cache")?,
            hashes: HashMap::new(),
        })
    }

    /// Resolve references after their predecessor commands have completed.
    pub fn resolve_all(
        &mut self,
        artifacts: &BTreeMap<Str, Artifact>,
        results: &[BTreeMap<Str, PathBuf>],
    ) -> BakeryResult<BTreeMap<Str, PathBuf>> {
        artifacts
            .iter()
            .map(|(name, artifact)| Ok((name.clone(), self.resolve(artifact, results)?)))
            .collect()
    }

    /// Hash each immutable artifact at most once per execution.
    pub fn hash(&mut self, path: &Path) -> BakeryResult<String> {
        if let Some(hash) = self.hashes.get(path) {
            return Ok(hash.clone());
        }
        let hash = artifact_hash(path)?;
        self.hashes.insert(path.to_owned(), hash.clone());
        Ok(hash)
    }

    fn resolve(
        &mut self,
        artifact: &Artifact,
        results: &[BTreeMap<Str, PathBuf>],
    ) -> BakeryResult<PathBuf> {
        match &artifact.source {
            ArtifactLocation::Path(path) => Ok(path.clone()),
            ArtifactLocation::Inline(bytes) => {
                let destination = self
                    .trees
                    .join(format!("data-{}", hex::encode(Sha256::digest(bytes))));
                if !destination.is_file() {
                    let mut temporary = tempfile::NamedTempFile::new_in(&self.trees)
                        .whatever("unable to create graph input")?;
                    temporary
                        .write_all(bytes)
                        .whatever("unable to materialize graph input")?;
                    temporary
                        .as_file()
                        .set_times(fs::FileTimes::new().set_modified(UNIX_EPOCH))
                        .whatever("unable to set graph input timestamp")?;
                    temporary
                        .persist(&destination)
                        .whatever("unable to publish graph input")?;
                }
                Ok(destination)
            }
            ArtifactLocation::Output { task, name } => Ok(results[*task][name].clone()),
            ArtifactLocation::Select { source, path } => {
                let root = self.resolve(source, results)?;
                let selected =
                    fs::canonicalize(root.join(path)).whatever("unable to select artifact path")?;
                if !selected.starts_with(&root) {
                    bail!("artifact selection escapes its source directory");
                }
                let metadata =
                    fs::metadata(&selected).whatever("unable to inspect selected artifact")?;
                if !match artifact.kind {
                    ArtifactKind::File => metadata.is_file(),
                    ArtifactKind::Directory => metadata.is_dir(),
                } {
                    bail!("selected artifact has an incompatible kind");
                }
                Ok(selected)
            }
            ArtifactLocation::Unpack(archive) => {
                let archive = self.resolve(archive, results)?;
                let hash = self.hash(&archive)?;
                let destination = self.trees.join(format!("layer-{hash}"));
                if !destination.is_dir() {
                    let temporary = tempfile::tempdir_in(&self.trees)
                        .whatever("unable to create layer extraction workspace")?;
                    crate::oven::archive::extract(&archive, temporary.path())?;
                    for path in [
                        "roots/system",
                        "roots/boot",
                        "roots/config",
                        "artifacts",
                        "bsp",
                    ] {
                        let path = temporary.path().join(path);
                        if !path.exists() {
                            fs::create_dir_all(&path)
                                .whatever("unable to create layer artifact directory")?;
                            fs::File::open(&path)
                                .whatever("unable to open layer artifact directory")?
                                .set_times(fs::FileTimes::new().set_modified(UNIX_EPOCH))
                                .whatever("unable to set layer artifact timestamp")?;
                        }
                    }
                    fs::rename(temporary.path(), &destination)
                        .whatever("unable to publish extracted layer artifacts")?;
                }
                Ok(destination)
            }
            ArtifactLocation::Tree(entries) => {
                let mut resolved = BTreeMap::new();
                let mut hashes = BTreeMap::new();
                for (target, artifact) in entries {
                    let source = self.resolve(artifact, results)?;
                    hashes.insert(target.clone(), self.hash(&source)?);
                    resolved.insert(target.clone(), source);
                }
                let mut key = Sha256::new();
                key.update(b"rugix-artifact-tree-v1\0");
                key.update(
                    serde_json::to_vec(&hashes)
                        .whatever("unable to encode artifact tree identity")?,
                );
                let destination = self.trees.join(hex::encode(key.finalize()));
                if destination.is_dir() {
                    return Ok(destination);
                }
                let temporary = tempfile::tempdir_in(&self.trees)
                    .whatever("unable to create artifact tree workspace")?;
                let root = temporary.path().join("tree");
                fs::create_dir(&root).whatever("unable to create artifact tree root")?;
                let mut generated = vec![root.clone()];
                for (target, source) in resolved {
                    let target = root.join(target);
                    let mut parent = target.parent().expect("tree entries have a parent");
                    while !parent.exists() {
                        generated.push(parent.to_owned());
                        parent = parent.parent().expect("tree parents remain below the root");
                    }
                    fs::create_dir_all(target.parent().expect("tree entries have a parent"))
                        .whatever("unable to create artifact tree parent")?;
                    run!([
                        "cp",
                        "--archive",
                        "--no-target-directory",
                        "--reflink=auto",
                        "--sparse=always",
                        "--",
                        source,
                        target
                    ])
                    .whatever("unable to place artifact in tree")?;
                }
                // Generated directories have no source metadata. Keep their identity
                // stable.
                for directory in generated {
                    fs::set_permissions(&directory, fs::Permissions::from_mode(0o755))
                        .whatever("unable to set artifact tree permissions")?;
                    fs::File::open(directory)
                        .whatever("unable to open artifact tree directory")?
                        .set_times(fs::FileTimes::new().set_modified(UNIX_EPOCH))
                        .whatever("unable to set artifact tree timestamp")?;
                }
                fs::rename(root, &destination).whatever("unable to publish artifact tree")?;
                Ok(destination)
            }
        }
    }
}
