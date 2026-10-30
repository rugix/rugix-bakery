//! Release metadata and layout conversion for predefined image pipelines.

use crate::config::bsp::BspConfig;
use crate::config::images::{ImageLayout, ImagePartition, RawBlob};
use crate::BakeryResult;
use reportify::ResultExt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ReleaseInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub name: String,
    pub release: SystemReleaseInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemReleaseInfo {
    pub id: String,
    pub version: String,
}

/// Load a `BspConfig` from a `rugix-bsp.toml` file.
pub fn load_bsp_config(path: &Path) -> BakeryResult<BspConfig> {
    info!("Loading BSP config from `{}`.", path.display());
    let content = fs::read_to_string(path).whatever("unable to read rugix-bsp.toml")?;
    toml::from_str(&content).whatever("unable to parse rugix-bsp.toml")
}

/// Convert a `BspConfig` disk layout into an `ImageLayout`.
pub fn bsp_to_image_layout(bsp: &BspConfig) -> ImageLayout {
    let dl = &bsp.image.layout;

    let raw_blobs: Vec<RawBlob> = dl
        .raw_blobs
        .iter()
        .flatten()
        .map(|b| RawBlob {
            file: b.file.clone(),
            offset: b.offset,
            size: b.size,
        })
        .collect();

    let partitions: Vec<ImagePartition> = dl
        .partitions
        .iter()
        .flatten()
        .map(|p| ImagePartition {
            size: p.size,
            filesystem: p.filesystem.clone(),
            root: p.root.clone(),
            ty: p.ty,
            bootable: p.bootable,
        })
        .collect();

    ImageLayout {
        ty: Some(dl.ty),
        raw_blobs: Some(raw_blobs.into()),
        partitions: Some(partitions.into()),
    }
}

/// Release identity shared by predefined and repository-defined pipelines.
pub fn release_metadata(
    release: &ReleaseInfo,
    system: &str,
    source_date_epoch: u64,
) -> BakeryResult<SystemInfo> {
    let timestamp = jiff::Timestamp::from_second(
        i64::try_from(source_date_epoch).whatever("source date is out of range")?,
    )
    .whatever("source date is out of range")?;
    let version = release
        .system_version
        .clone()
        .unwrap_or_else(|| timestamp.strftime("%Y%m%d%H%M%S").to_string());
    let id = release
        .system_id
        .clone()
        .unwrap_or_else(|| format!("{system}@{version}"));
    Ok(SystemInfo {
        name: system.to_owned(),
        release: SystemReleaseInfo { id, version },
    })
}
