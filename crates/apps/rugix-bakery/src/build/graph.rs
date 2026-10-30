//! Deferred graph expansion for plans whose shape depends on predecessor artifacts.

use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;

use sidex::types::Str;

use crate::config::rules::ArtifactKind;
use crate::BakeryResult;

use super::{Artifact, ArtifactLocation, Plan, Planner, Task};

type Expansion = dyn Fn(&BTreeMap<Str, PathBuf>) -> BakeryResult<Plan>;

/// A planning boundary within the same executor and cache session.
pub struct GraphTask {
    pub inputs: BTreeMap<Str, Artifact>,
    pub expand: Box<Expansion>,
}

impl fmt::Debug for GraphTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphTask")
            .field("inputs", &self.inputs)
            .finish_non_exhaustive()
    }
}

impl Planner {
    /// Defer planning until all inputs exist; execute the resulting nodes normally.
    pub fn defer(
        &mut self,
        inputs: BTreeMap<Str, Artifact>,
        outputs: BTreeMap<Str, ArtifactKind>,
        expand: impl Fn(&BTreeMap<Str, PathBuf>) -> BakeryResult<Plan> + 'static,
    ) -> BTreeMap<Str, Artifact> {
        let task = self.plan.tasks.len();
        self.plan.tasks.push(Task::Graph(GraphTask {
            inputs,
            expand: Box::new(expand),
        }));
        outputs
            .into_iter()
            .map(|(name, kind)| {
                (
                    name.clone(),
                    Artifact {
                        kind,
                        source: ArtifactLocation::Output { task, name },
                    },
                )
            })
            .collect()
    }
}
