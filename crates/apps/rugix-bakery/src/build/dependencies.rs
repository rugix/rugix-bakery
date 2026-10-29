//! Register and fingerprint project dependencies discovered by rules and recipes.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::{fs, io};

use reportify::{bail, ResultExt};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::BakeryResult;

use super::cache::artifact_hash;

/// Recorded project paths and their content identities, including absent paths.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDependencies(BTreeMap<PathBuf, String>);

impl ProjectDependencies {
    /// Capture newline-delimited paths relative to the project directory.
    pub fn capture(project: &Path, registrations: &Path) -> BakeryResult<Self> {
        let registrations = fs::read_to_string(registrations)
            .whatever("unable to read project dependency registrations")?;
        Self::snapshot(
            project,
            registrations
                .lines()
                .filter(|line| !line.is_empty())
                .map(Path::new),
        )
    }

    /// Load an internal cache record. Older builds without a record must run once.
    pub fn load(path: &Path) -> BakeryResult<Option<Self>> {
        match fs::read(path) {
            Ok(bytes) => Ok(Some(
                serde_json::from_slice(&bytes).whatever("unable to decode project dependencies")?,
            )),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error).whatever("unable to read project dependencies"),
        }
    }

    /// Atomically replace a dependency record only after a successful build.
    pub fn save(&self, path: &Path) -> BakeryResult<()> {
        let parent = path.parent().expect("dependency records have a parent");
        fs::create_dir_all(parent).whatever("unable to create dependency record directory")?;
        let temporary = tempfile::NamedTempFile::new_in(parent)
            .whatever("unable to create project dependency record")?;
        fs::write(temporary.path(), self.encode()?)
            .whatever("unable to write project dependencies")?;
        temporary
            .persist(path)
            .whatever("unable to publish project dependencies")?;
        Ok(())
    }

    /// Recheck the previous execution's paths before reusing its outputs.
    pub fn is_current(&self, project: &Path) -> BakeryResult<bool> {
        Ok(*self == Self::snapshot(project, self.0.keys().map(PathBuf::as_path))?)
    }

    /// Add discovered dependencies to a rule's otherwise static cache identity.
    pub fn cache_key(&self, base: &str) -> BakeryResult<String> {
        let mut hash = Sha256::new();
        hash.update(base.as_bytes());
        hash.update(self.encode()?);
        Ok(hex::encode(hash.finalize()))
    }

    fn encode(&self) -> BakeryResult<Vec<u8>> {
        serde_json::to_vec(self).whatever("unable to encode project dependencies")
    }

    fn snapshot<'a>(project: &Path, paths: impl Iterator<Item = &'a Path>) -> BakeryResult<Self> {
        let project = fs::canonicalize(project).whatever("unable to resolve project directory")?;
        let mut dependencies = BTreeMap::new();
        for path in paths {
            if path.as_os_str().is_empty()
                || path
                    .components()
                    .any(|part| !matches!(part, Component::Normal(_) | Component::CurDir))
            {
                bail!("project dependencies must be relative paths without '..'");
            }
            let path: PathBuf = path.components().collect();
            if !dependencies.contains_key(&path) {
                dependencies.insert(path.clone(), fingerprint(&project, &project.join(&path))?);
            }
        }
        Ok(Self(dependencies))
    }
}

/// Hash trees and the targets of contained symlinks without following cycles.
fn fingerprint(project: &Path, path: &Path) -> BakeryResult<String> {
    let mut pending = vec![path.to_owned()];
    let mut contents = BTreeMap::new();
    let mut visited = BTreeSet::new();
    while let Some(path) = pending.pop() {
        let relative = path
            .strip_prefix(project)
            .expect("dependency paths stay in project")
            .to_owned();
        if contents.contains_key(&relative) {
            continue;
        }
        let resolved = resolve_dependency(project, &path, 0)?;
        let link = match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.is_symlink() => Some(artifact_hash(&path)?),
            Ok(_) => None,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
                ) =>
            {
                None
            }
            Err(error) => return Err(error).whatever("unable to inspect project dependency"),
        };
        let Some(resolved) = resolved else {
            contents.insert(relative, (None, link));
            continue;
        };
        contents.insert(
            relative,
            (
                Some((
                    resolved
                        .strip_prefix(project)
                        .expect("target stays in project")
                        .to_owned(),
                    artifact_hash(&resolved)?,
                )),
                link,
            ),
        );
        if visited.insert(resolved.clone()) && resolved.is_dir() {
            collect_symlinks(&resolved, &mut pending)?;
        }
    }
    let bytes =
        serde_json::to_vec(&contents).whatever("unable to encode dependency content identity")?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

/// Find symlinks in real directories; their targets are hashed separately.
fn collect_symlinks(directory: &Path, links: &mut Vec<PathBuf>) -> BakeryResult<()> {
    for entry in fs::read_dir(directory).whatever("unable to read project dependency directory")? {
        let entry = entry.whatever("unable to read project dependency entry")?;
        let kind = entry
            .file_type()
            .whatever("unable to inspect project dependency entry")?;
        if kind.is_symlink() {
            links.push(entry.path());
        } else if kind.is_dir() {
            collect_symlinks(&entry.path(), links)?;
        }
    }
    Ok(())
}

/// Resolve links component by component, validating targets even when they are absent.
fn resolve_dependency(project: &Path, path: &Path, links: usize) -> BakeryResult<Option<PathBuf>> {
    if links >= 40 {
        bail!("project dependency has too many symlinks");
    }
    let relative = path
        .strip_prefix(project)
        .whatever("project dependency escapes the project directory")?;
    let mut resolved = project.to_owned();
    let mut components = relative.components();
    while let Some(component) = components.next() {
        match component {
            Component::CurDir => continue,
            Component::ParentDir => {
                if resolved == project {
                    bail!("project dependency escapes the project directory");
                }
                resolved.pop();
                continue;
            }
            Component::Normal(part) => resolved.push(part),
            _ => bail!("invalid project dependency path"),
        }
        match fs::symlink_metadata(&resolved) {
            Ok(metadata) if metadata.is_symlink() => {
                let target = fs::read_link(&resolved)
                    .whatever("unable to read project dependency symlink")?;
                let target = if target.is_absolute() {
                    target
                } else {
                    resolved
                        .parent()
                        .expect("dependency has a parent")
                        .join(target)
                };
                resolve_dependency(project, &target.join(components.as_path()), links + 1)?;
                break;
            }
            Ok(_) => {}
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
                ) => {}
            Err(error) => return Err(error).whatever("unable to inspect project dependency"),
        }
    }
    // Keep the original traversal: missing/.. and file/.. are still absent paths.
    match fs::canonicalize(path) {
        Ok(resolved) => {
            if !resolved.starts_with(project) {
                bail!("project dependency escapes the project directory");
            }
            Ok(Some(resolved))
        }
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
            ) =>
        {
            Ok(None)
        }
        Err(error) => Err(error).whatever("unable to resolve project dependency"),
    }
}
