//! Build and publish image assembly from a graph containing its layers.

use std::collections::BTreeMap;
use std::path::Path;

use crate::build::{self, Artifact, ExecutionContext, Executor, Planner};
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
    context: ExecutionContext,
    selection: &super::ArtifactSelection,
) -> BakeryResult<()> {
    let inputs = planner.release(&layer, config.architecture, release, system);
    let context_artifact = inputs["context"].clone();
    let standard = if config.target.is_some() || config.image.is_some() {
        super::pipeline::plan(&mut planner, inputs.clone(), config, bundle_options)
    } else {
        BTreeMap::new()
    };
    let mut outputs = planner.system_graph(config, inputs, standard)?;
    outputs.insert("build-context.json".into(), context_artifact);
    selection.select(&mut outputs)?;
    let plan = planner.finish(outputs);
    let mut executor = Executor::for_project(project, context)?;
    let artifacts = executor.execute(&plan)?;
    build::publish(&artifacts, output, project.dir())
}
