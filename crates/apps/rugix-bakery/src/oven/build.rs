//! Build and publish image assembly from a graph containing its layers.

use std::collections::BTreeMap;
use std::path::Path;

use reportify::bail;

use crate::build::{self, Artifact, Executor, Planner};
use crate::config::systems::SystemConfig;
use crate::oven::system::ReleaseInfo;
use crate::project::ProjectRef;
use crate::BakeryResult;

/// Execute layers, release metadata, and image rules in one graph.
#[tracing::instrument(skip_all)]
pub fn assemble(
    project: &ProjectRef,
    mut planner: Planner,
    layer: BTreeMap<sidex::types::Str, Artifact>,
    config: &SystemConfig,
    bundle_options: Option<&super::BundleOpts>,
    release: &ReleaseInfo,
    system: &str,
    output: &Path,
    source_date_epoch: Option<u64>,
) -> BakeryResult<()> {
    let inputs = planner.release(
        &layer,
        config.architecture,
        release,
        system,
        source_date_epoch,
    );
    let context = inputs["context"].clone();
    let mut outputs = if let Some(invocation) = &config.build {
        planner.invocation(invocation, inputs)?
    } else {
        super::pipeline::plan(&mut planner, inputs, config, bundle_options)
    };
    if !outputs.contains_key("image") {
        bail!("system assembly must export an image artifact");
    }
    if outputs
        .insert("build-context.json".into(), context)
        .is_some()
    {
        bail!("assembly output build-context.json is reserved for release metadata");
    }
    let plan = planner.finish(outputs);
    let mut executor = Executor::for_project(project)?;
    let artifacts = executor.execute(&plan)?;
    if !artifacts["image"].is_file() {
        bail!("system assembly image must be a file");
    }
    let mut publication = BTreeMap::new();
    for (name, source) in artifacts {
        let filename = match name.as_str() {
            "image" => "system.img",
            "bundle" => "system.rugixb",
            other => other,
        };
        if publication.insert(filename.into(), source).is_some() {
            bail!("assembly outputs collide at destination {filename:?}");
        }
    }
    build::publish(&publication, output, project.dir())
}
