//! Recipe build directories shared with the existing layer customization code.

use crate::project::ProjectRef;
use std::path::PathBuf;

pub struct LayerContext {
    pub project: ProjectRef,
    pub build_dir: PathBuf,
    pub output_dir: PathBuf,
}
