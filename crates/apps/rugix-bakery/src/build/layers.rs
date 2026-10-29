//! Plan layer and mixin nodes and execute their existing recipe schedules.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use reportify::{bail, whatever, ResultExt};
use sidex::types::{Map, Str};

use crate::config::recipes::ParameterValue;
use crate::config::rules::{ArtifactKind, ArtifactSource, RuleInvocation};
use crate::config::systems::Architecture;
use crate::oven::customize::{self, RecipeFragment, RecipeJob};
use crate::oven::ParameterOverrides;
use crate::project::library::MixinIdx;
use crate::project::repositories::RepositoryIdx;
use crate::project::ProjectRef;
use crate::utils::caching::Hasher;
use crate::BakeryResult;

use super::cache::artifact_hash;
use super::planner::Scope;
use super::{validate_name, Artifact, ArtifactLocation, Plan, Planner, Task};

#[derive(Clone)]
pub(crate) struct LayerOptions {
    pub project: ProjectRef,
    pub source_date_epoch: u64,
    pub explicit_source_date_epoch: Option<u64>,
    pub parameters: ParameterOverrides,
}

/// One layer import or customization, scheduled after its artifact prerequisites.
#[derive(Debug)]
pub struct LayerTask {
    name: String,
    identity: String,
    architecture: Architecture,
    jobs: Vec<RecipeJob>,
    url: Option<String>,
    pub inputs: BTreeMap<Str, Artifact>,
    options: LayerOptions,
}

impl std::fmt::Debug for LayerOptions {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LayerOptions")
            .finish_non_exhaustive()
    }
}

impl Planner {
    /// Enable existing layers in this graph without loading unrelated layer files yet.
    pub fn with_layers(
        mut self,
        project: &ProjectRef,
        source_date_epoch: Option<u64>,
        parameters: &ParameterOverrides,
    ) -> Self {
        self.layer_options = Some(LayerOptions {
            project: project.clone(),
            source_date_epoch: source_date_epoch
                .unwrap_or_else(|| jiff::Timestamp::now().as_second() as u64),
            explicit_source_date_epoch: source_date_epoch,
            parameters: parameters.clone(),
        });
        self
    }

    /// Finish a graph assembled through the layer and invocation frontends.
    pub fn finish(mut self, outputs: BTreeMap<Str, Artifact>) -> Plan {
        self.plan.outputs = outputs;
        self.plan
    }

    /// Append a repository invocation to an existing graph.
    pub fn invocation(
        &mut self,
        invocation: &RuleInvocation,
        inputs: BTreeMap<Str, Artifact>,
    ) -> BakeryResult<BTreeMap<Str, Artifact>> {
        let mut scope = Scope::new(self.repositories.root_repository);
        scope.inputs = inputs;
        self.invoke(&mut scope, invocation)
    }

    /// Resolve parents and recipe inputs before appending the layer node.
    pub fn layer(
        &mut self,
        repository: RepositoryIdx,
        reference: &str,
        architecture: Architecture,
        inputs: BTreeMap<Str, Artifact>,
        parameters: BTreeMap<Str, ParameterValue>,
    ) -> BakeryResult<BTreeMap<Str, Artifact>> {
        let options = self
            .layer_options
            .clone()
            .ok_or_else(|| whatever!("layer execution requires a project"))?;
        let library = options.project.library()?;
        let index = library
            .lookup_layer(repository, reference)
            .ok_or_else(|| whatever!("unable to find layer {reference:?}"))?;
        let shared = inputs.is_empty() && parameters.is_empty();
        let key = (index, architecture);
        if shared {
            if let Some(outputs) = self.layer_completed.get(&key) {
                return Ok(outputs.clone());
            }
        }
        if !self.layer_visiting.insert(key) {
            bail!("layer dependency cycle involving {reference:?}");
        }
        let layer = &library.layers[index];
        let config = layer
            .config(architecture)
            .ok_or_else(|| whatever!("no layer configuration for architecture {architecture}"))?;
        let mut identity = Hasher::new();
        identity.push("layer", &layer.name);
        identity.push(
            "repository",
            self.repositories[layer.repo].source.id.as_str(),
        );
        identity.push("arch", architecture.as_str());
        crate::oven::push_parameter_overrides(&mut identity, &options.parameters, &library);
        let parent = if let Some(url) = &config.url {
            identity.push("url", url);
            if config.rules.as_ref().is_some_and(|rules| !rules.is_empty())
                || config
                    .inputs
                    .as_ref()
                    .is_some_and(|inputs| !inputs.is_empty())
            {
                bail!("imported layers cannot define recipe inputs or prerequisite rules");
            }
            BTreeMap::new()
        } else if let Some(parent) = &config.parent {
            identity.push("parent", parent.as_str());
            self.layer(
                layer.repo,
                parent,
                architecture,
                BTreeMap::new(),
                BTreeMap::new(),
            )?
        } else if config.root.unwrap_or(false) {
            identity.push("bare", "true");
            BTreeMap::new()
        } else {
            bail!("layer {reference:?} must specify url, parent, or root");
        };
        let jobs = if config.url.is_some() {
            Vec::new()
        } else {
            let jobs = customize::recipe_schedule(
                layer.repo,
                RecipeFragment::from_layer(config),
                &library,
                &options.parameters,
            )?;
            if jobs.is_empty() {
                bail!("layer {reference:?} must have recipes");
            }
            jobs
        };
        let bindings = self.recipe_inputs(
            layer.repo,
            architecture,
            &jobs,
            config.rules.as_ref(),
            config.inputs.as_ref(),
            &parent,
            inputs,
            parameters,
        )?;
        let outputs = self.append_layer(
            LayerTask {
                name: layer.name.clone(),
                identity: identity.finalize(),
                architecture,
                jobs,
                url: config.url.as_ref().map(ToString::to_string),
                inputs: bindings,
                options,
            },
            &parent,
        );
        self.layer_visiting.remove(&key);
        if shared {
            self.layer_completed.insert(key, outputs.clone());
        }
        Ok(outputs)
    }

    /// Apply a resolved mixin to its predecessor in the same graph.
    pub fn mixin(
        &mut self,
        index: MixinIdx,
        architecture: Architecture,
        parent: BTreeMap<Str, Artifact>,
    ) -> BakeryResult<BTreeMap<Str, Artifact>> {
        let options = self
            .layer_options
            .clone()
            .ok_or_else(|| whatever!("mixin execution requires a project"))?;
        let library = options.project.library()?;
        let mixin = &library.mixins[index];
        let config = mixin
            .config(architecture)
            .ok_or_else(|| whatever!("no mixin configuration for architecture {architecture}"))?;
        let jobs = customize::recipe_schedule(
            mixin.repo,
            RecipeFragment::from_mixin(config),
            &library,
            &options.parameters,
        )?;
        if jobs.is_empty() {
            if config
                .inputs
                .as_ref()
                .is_some_and(|inputs| !inputs.is_empty())
            {
                bail!("empty mixin cannot bind recipe inputs");
            }
            return Ok(parent);
        }
        let mut identity = Hasher::new();
        identity.push("mixin", &mixin.name);
        identity.push(
            "repository",
            self.repositories[mixin.repo].source.id.as_str(),
        );
        identity.push("arch", architecture.as_str());
        crate::oven::push_parameter_overrides(&mut identity, &options.parameters, &library);
        let inputs = self.recipe_inputs(
            mixin.repo,
            architecture,
            &jobs,
            config.rules.as_ref(),
            config.inputs.as_ref(),
            &parent,
            BTreeMap::new(),
            BTreeMap::new(),
        )?;
        Ok(self.append_layer(
            LayerTask {
                name: format!("mixin:{}", mixin.name),
                identity: identity.finalize(),
                architecture,
                jobs,
                url: None,
                inputs,
                options,
            },
            &parent,
        ))
    }

    fn recipe_inputs(
        &mut self,
        repository: RepositoryIdx,
        architecture: Architecture,
        jobs: &[RecipeJob],
        rules: Option<&Map<Str, RuleInvocation>>,
        bindings: Option<&Map<Str, Map<Str, ArtifactSource>>>,
        parent: &BTreeMap<Str, Artifact>,
        inputs: BTreeMap<Str, Artifact>,
        mut parameters: BTreeMap<Str, ParameterValue>,
    ) -> BakeryResult<BTreeMap<Str, Artifact>> {
        let library = self
            .layer_options
            .as_ref()
            .expect("layer options are set")
            .project
            .library()?;
        let mut supplied = BTreeMap::new();
        for (reference, inputs) in bindings.into_iter().flatten() {
            let recipe = library.try_lookup(repository, reference)?;
            let recipe = &library.recipes[recipe];
            let index = jobs
                .iter()
                .position(|job| std::sync::Arc::ptr_eq(recipe, &job.recipe))
                .ok_or_else(|| {
                    whatever!("input bindings refer to inactive recipe {reference:?}")
                })?;
            if supplied.insert(index, inputs).is_some() {
                bail!("duplicate input bindings for recipe {reference:?}");
            }
        }
        let mut scope = Scope::with_rules(repository, rules)?;
        scope.inputs = parent
            .iter()
            .filter(|(name, _)| name.as_str() != "archive")
            .map(|(name, artifact)| (name.clone(), artifact.clone()))
            .collect();
        for (name, artifact) in inputs {
            if scope.inputs.insert(name.clone(), artifact).is_some() {
                bail!("layer input {name:?} conflicts with a parent artifact");
            }
        }
        if parameters
            .insert(
                "architecture".into(),
                ParameterValue::String(architecture.as_str().into()),
            )
            .is_some()
        {
            bail!("architecture is a reserved layer parameter");
        }
        scope.parameters = parameters;
        let mut resolved = BTreeMap::new();
        for (index, job) in jobs.iter().enumerate() {
            let declared = job.recipe.config.inputs.as_ref();
            let bound = supplied.get(&index).copied();
            if let Some(bound) = bound {
                for name in bound.keys() {
                    if !declared.is_some_and(|declared| declared.contains_key(name)) {
                        bail!("unknown input {name:?} for recipe {}", job.recipe.name);
                    }
                }
            }
            let mut recipe_scope =
                Scope::with_rules(job.recipe.repository, job.recipe.config.rules.as_ref())?;
            recipe_scope.inputs = scope.inputs.clone();
            recipe_scope.parameters = BTreeMap::from([(
                "architecture".into(),
                ParameterValue::String(architecture.as_str().into()),
            )]);
            recipe_scope.parameters.extend(
                job.parameters
                    .iter()
                    .map(|(name, value)| (name.clone().into(), value.clone())),
            );
            if let Some(defaults) = &job.recipe.config.input_defaults {
                for name in defaults.keys() {
                    if !declared.is_some_and(|declared| declared.contains_key(name)) {
                        bail!(
                            "unknown default input {name:?} for recipe {}",
                            job.recipe.name
                        );
                    }
                }
                recipe_scope.input_defaults = defaults.clone();
            }
            // Resolve enclosing bindings before defaults, regardless of declaration order.
            for name in declared.into_iter().flatten().map(|(name, _)| name) {
                validate_name(name)?;
                if let Some(source) = bound.and_then(|bound| bound.get(name)) {
                    recipe_scope
                        .inputs
                        .insert(name.clone(), self.artifact(&mut scope, source, false)?);
                }
            }
            let mut tree = BTreeMap::new();
            for (name, kind) in declared.into_iter().flatten() {
                let artifact = self.input(&mut recipe_scope, name).message(|| {
                    format!(
                        "while resolving input {name:?} for recipe {}",
                        job.recipe.name
                    )
                })?;
                if artifact.kind != kind.kind() {
                    bail!(
                        "input {name:?} for recipe {} has an incompatible artifact kind",
                        job.recipe.name
                    );
                }
                tree.insert(PathBuf::from(name.as_str()), artifact);
            }
            resolved.insert(
                index.to_string().into(),
                Artifact {
                    kind: ArtifactKind::Directory,
                    source: ArtifactLocation::Tree(tree),
                },
            );
        }
        Ok(resolved)
    }

    fn append_layer(
        &mut self,
        mut task: LayerTask,
        parent: &BTreeMap<Str, Artifact>,
    ) -> BTreeMap<Str, Artifact> {
        if let Some(parent) = parent.get("archive") {
            task.inputs.insert("parent".into(), parent.clone());
        }
        let index = self.plan.tasks.len();
        self.plan.tasks.push(Task::Layer(task));
        layer_outputs(index)
    }
}

impl LayerTask {
    /// Retain archive compatibility while input identities select an immutable cache
    /// variant.
    pub fn execute(
        &self,
        inputs: &BTreeMap<Str, PathBuf>,
        hashes: &BTreeMap<Str, String>,
        runtime: Option<&str>,
        workspace: Option<&Path>,
    ) -> BakeryResult<BTreeMap<Str, PathBuf>> {
        let mut identity = Hasher::new();
        identity.push("layer-graph-v1", &self.identity);
        identity.push("runtime", runtime.unwrap_or("unidentified-bakery"));
        if let Some(epoch) = self.options.explicit_source_date_epoch {
            identity.push("source-date-epoch", epoch.to_string());
        }
        for (name, hash) in hashes {
            identity.push(name, hash);
        }
        for (index, job) in self.jobs.iter().enumerate() {
            let directory = fs::canonicalize(&job.recipe.path)
                .whatever("unable to resolve recipe implementation")?;
            identity.push(&format!("recipe:{index}"), artifact_hash(&directory)?);
            let parameters: BTreeMap<_, _> = job.parameters.iter().collect();
            identity.push(
                &format!("parameters:{index}"),
                serde_json::to_vec(&parameters).whatever("unable to encode recipe parameters")?,
            );
        }
        // Uncached tasks need distinct paths even when their declared inputs match.
        // The executor retains their workspaces until all outputs are published.
        let relative = workspace
            .map(Path::to_owned)
            .unwrap_or_else(|| PathBuf::from(".rugix/layers").join(identity.finalize()));
        let target = self
            .options
            .project
            .dir()
            .join(&relative)
            .join("system.tar");
        fs::create_dir_all(target.parent().expect("layer archive has a parent"))
            .whatever("unable to create layer cache")?;
        if let Some(url) = &self.url {
            if runtime.is_none() || !target.is_file() {
                crate::oven::extract(&self.options.project, url, &target)?;
            }
        } else {
            if let Err(error) = crate::utils::emulation::prepare(self.architecture) {
                tracing::debug!(?error, "automatic target emulation is unavailable");
            }
            customize::customize(
                &self.options.project,
                self.architecture,
                &self.name,
                &self.jobs,
                inputs,
                inputs.get("parent").map(PathBuf::as_path),
                &target,
                &relative,
                self.options.source_date_epoch,
                runtime.is_some(),
            )?;
        }
        Ok(BTreeMap::from([("archive".into(), target)]))
    }
}

/// Expose directories lazily, so archive-only consumers do not extract every parent.
fn layer_outputs(task: usize) -> BTreeMap<Str, Artifact> {
    let archive = Artifact {
        kind: ArtifactKind::File,
        source: ArtifactLocation::Output {
            task,
            name: "archive".into(),
        },
    };
    let unpacked = Artifact {
        kind: ArtifactKind::Directory,
        source: ArtifactLocation::Unpack(Box::new(archive.clone())),
    };
    let mut outputs = BTreeMap::from([("archive".into(), archive)]);
    for (name, path) in [
        ("rootfs", "roots/system"),
        ("boot", "roots/boot"),
        ("config", "roots/config"),
        ("artifacts", "artifacts"),
        ("bsp", "bsp"),
    ] {
        outputs.insert(
            name.into(),
            Artifact {
                kind: ArtifactKind::Directory,
                source: ArtifactLocation::Select {
                    source: Box::new(unpacked.clone()),
                    path: path.into(),
                },
            },
        );
    }
    outputs
}
