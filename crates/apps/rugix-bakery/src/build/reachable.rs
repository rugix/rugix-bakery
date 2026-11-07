//! Compute task and output dependencies without renumbering planned artifacts.

use super::{Artifact, ArtifactLocation, Plan, Task};
use sidex::types::Str;
use std::collections::BTreeSet;

/// Requested output ports for each task, in original plan order.
pub fn reachable(plan: &Plan) -> Vec<BTreeSet<Str>> {
    let mut needed = vec![BTreeSet::new(); plan.tasks.len()];
    for output in plan.outputs.values() {
        visit(output, &mut needed);
    }
    for (index, task) in plan.tasks.iter().enumerate().rev() {
        if needed[index].is_empty() {
            continue;
        }
        match task {
            Task::Run(task) => {
                for artifact in task.inputs.values().chain(task.secrets.values()) {
                    visit(artifact, &mut needed);
                }
            }
            Task::Graph(task) => {
                for artifact in task.inputs.values() {
                    visit(artifact, &mut needed);
                }
            }
            Task::Layer(task) => {
                for artifact in task.inputs.values() {
                    visit(artifact, &mut needed);
                }
            }
            Task::Release(task) => visit(&task.archive, &mut needed),
        }
    }
    needed
}

fn visit(artifact: &Artifact, needed: &mut [BTreeSet<Str>]) {
    match &artifact.source {
        ArtifactLocation::Output { task, name } => {
            needed[*task].insert(name.clone());
        }
        ArtifactLocation::Select { source, .. } | ArtifactLocation::Unpack(source) => {
            visit(source, needed)
        }
        ArtifactLocation::Tree(entries) => {
            for artifact in entries.values() {
                visit(artifact, needed);
            }
        }
        ArtifactLocation::Inline(_) | ArtifactLocation::Path(_) => {}
    }
}
