//! Frozen and extracted representations of Bakery layers.
//!
//! [`FrozenLayer::unfreeze`] restores an archived layer as a temporary [`Layer`].

use std::path::{Path, PathBuf};

use reportify::ResultExt;
use tempfile::TempDir;
use tracing::info;

use crate::oven::archive;
use crate::project::ProjectRef;
use crate::utils::caching::{mtime, ModificationTime};
use crate::BakeryResult;

#[derive(Debug)]
pub struct FrozenLayer {
    name: String,
    path: PathBuf,
}

impl FrozenLayer {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn last_modified(&self) -> BakeryResult<ModificationTime> {
        mtime(&self.path).whatever_with(|_| {
            format!(
                "unable to determine modification time of layer {}",
                self.name
            )
        })
    }

    pub fn unfreeze(&self) -> BakeryResult<Layer> {
        let tempdir = TempDir::new().whatever("unable to create temporary directory")?;
        info!("Extracting layer.");
        archive::extract(&self.path, tempdir.path())
            .whatever_with(|_| format!("failed to extract layer {}", self.name))?;
        Ok(Layer {
            name: self.name.clone(),
            tempdir,
        })
    }
}

#[derive(Debug)]
pub struct Layer {
    name: String,
    tempdir: TempDir,
}

impl Layer {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

impl AsRef<Path> for Layer {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

pub struct LayerContext {
    pub project: ProjectRef,
    pub build_dir: PathBuf,
    pub output_dir: PathBuf,
}
