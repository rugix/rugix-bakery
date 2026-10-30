//! Syft inventories with deterministic document identities and creation times.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use reportify::{whatever, ResultExt};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use uuid::Uuid;

use crate::config::build_context::BuildContext;
use crate::BakeryResult;

use super::{epoch_command, run};

/// Preserve Syft's third-party document fields while normalizing volatile metadata.
pub fn execute() -> BakeryResult<()> {
    let context: BuildContext =
        crate::config::load_json(Path::new("/run/rugix/bakery/work/inputs/context"))?;
    run(epoch_command("syft", context.source_date_epoch)?.args([
        "/run/rugix/bakery/work/inputs/tree",
        "--source-name",
        &context.system,
        "--source-version",
        &context.release_version,
        "--quiet",
        "-o",
        "spdx-json=/run/rugix/bakery/work/outputs/sbom.spdx.json",
        "-o",
        "cyclonedx-json=/run/rugix/bakery/work/outputs/sbom.cdx.json",
    ]))?;
    let timestamp = jiff::Timestamp::from_second(
        i64::try_from(context.source_date_epoch).whatever("source date is out of range")?,
    )
    .whatever("source date is out of range")?
    .to_string();
    normalize(
        Path::new("/run/rugix/bakery/work/outputs/sbom.spdx.json"),
        "creationInfo",
        "created",
        "documentNamespace",
        "https://rugix.org/sbom/",
        &timestamp,
    )?;
    normalize(
        Path::new("/run/rugix/bakery/work/outputs/sbom.cdx.json"),
        "metadata",
        "timestamp",
        "serialNumber",
        "urn:uuid:",
        &timestamp,
    )
}

/// A third-party JSON object, preserving fields outside the metadata we own.
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct Document(BTreeMap<String, Box<RawValue>>);

impl Document {
    fn set(&mut self, key: &str, value: &impl Serialize) -> BakeryResult<()> {
        let raw =
            serde_json::value::to_raw_value(value).whatever("unable to encode inventory field")?;
        self.0.insert(key.to_owned(), raw);
        Ok(())
    }
}

fn normalize(
    path: &Path,
    metadata_key: &str,
    timestamp_key: &str,
    identity_key: &str,
    identity_prefix: &str,
    timestamp: &str,
) -> BakeryResult<()> {
    let mut document: Document = crate::config::load_json(path)?;
    let raw = document
        .0
        .get(metadata_key)
        .ok_or_else(|| whatever!("software inventory has no {metadata_key}"))?;
    let mut metadata: Document =
        serde_json::from_str(raw.get()).whatever("invalid inventory metadata")?;
    metadata.set(timestamp_key, &timestamp)?;
    document.set(metadata_key, &metadata)?;
    document.set(identity_key, &"")?;
    let bytes = serde_json::to_vec(&document).whatever("unable to encode software inventory")?;
    let id = Uuid::new_v5(&Uuid::NAMESPACE_URL, &bytes);
    document.set(identity_key, &format!("{identity_prefix}{id}"))?;
    fs::write(
        path,
        serde_json::to_vec(&document).whatever("unable to encode software inventory")?,
    )
    .whatever("unable to write software inventory")
}
