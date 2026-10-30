//! Repository-defined artifact builds.
//!
//! A plan expands reusable compositions into commands before execution.
//! The existing layer builder supplies the initial filesystem artifacts.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use sidex::types::{Map, Str};

use reportify::{bail, whatever, ResultExt};

use crate::config::recipes::ParameterValue;
use crate::config::rules::{
    ArtifactKind, ArtifactSource, GraphRule, ParameterBinding, RuleDefinition, RuleExecution,
    RuleInvocation, RunRule,
};
use crate::project::envs::Envs;
use crate::project::repositories::{ProjectRepositories, RepositoryIdx};
use crate::BakeryResult;

/// An external artifact or a declared output of an earlier command.
#[derive(Clone, Debug)]
pub struct Artifact {
    pub kind: ArtifactKind,
    pub source: ArtifactLocation,
}

/// Location of an artifact in the expanded graph.
#[derive(Clone, Debug)]
pub enum ArtifactLocation {
    Inline(Vec<u8>),
    Path(PathBuf),
    Output {
        task: usize,
        name: Str,
    },
    Select {
        source: Box<Artifact>,
        path: PathBuf,
    },
    Tree(BTreeMap<PathBuf, Artifact>),
    Unpack(Box<Artifact>),
}

impl Artifact {
    /// Embed generated frontend data without exposing temporary file paths.
    pub fn json(value: &impl serde::Serialize) -> BakeryResult<Self> {
        Ok(Self {
            kind: ArtifactKind::File,
            source: ArtifactLocation::Inline(
                serde_json::to_vec(value).whatever("unable to encode graph input")?,
            ),
        })
    }

    /// Resolve an existing file or directory without traversing its contents.
    pub fn from_path(path: &Path) -> BakeryResult<Self> {
        let path = fs::canonicalize(path).whatever("unable to resolve input artifact")?;
        let metadata = fs::metadata(&path).whatever("unable to inspect input artifact")?;
        let kind = if metadata.is_file() {
            ArtifactKind::File
        } else if metadata.is_dir() {
            ArtifactKind::Directory
        } else {
            bail!("artifact must be a regular file or directory");
        };
        Ok(Self {
            kind,
            source: ArtifactLocation::Path(path),
        })
    }
}

/// A command whose dependencies have already been planned.
#[derive(Debug)]
pub struct RunTask {
    pub name: Str,
    pub directory: PathBuf,
    pub env: Option<PathBuf>,
    pub run: RunRule,
    pub inputs: BTreeMap<Str, Artifact>,
    pub parameters: BTreeMap<Str, ParameterValue>,
    pub secrets: BTreeMap<Str, Artifact>,
}

/// Operations scheduled together in one build graph.
#[derive(Debug)]
pub enum Task {
    Graph(super::graph::GraphTask),
    Run(RunTask),
    Layer(super::layers::LayerTask),
    Release(super::release::ReleaseTask),
}

/// A topologically ordered build and its exported artifacts.
#[derive(Debug, Default)]
pub struct Plan {
    pub tasks: Vec<Task>,
    pub outputs: BTreeMap<Str, Artifact>,
}

/// Expand repository rules and validate their interfaces without running containers.
pub struct Planner {
    pub(crate) repositories: Arc<ProjectRepositories>,
    envs: Envs,
    pub(crate) plan: Plan,
    pub(crate) layer_options: Option<super::layers::LayerOptions>,
    pub(crate) layer_visiting: HashSet<(
        crate::project::library::LayerIdx,
        crate::config::systems::Architecture,
    )>,
    pub(crate) layer_completed: HashMap<
        (
            crate::project::library::LayerIdx,
            crate::config::systems::Architecture,
        ),
        BTreeMap<Str, Artifact>,
    >,
    ancestry: Vec<PathBuf>,
}

impl Planner {
    pub fn new(repositories: Arc<ProjectRepositories>) -> BakeryResult<Self> {
        Ok(Self {
            envs: Envs::load(repositories.clone())?,
            repositories,
            plan: Plan::default(),
            layer_options: None,
            layer_visiting: HashSet::new(),
            layer_completed: HashMap::new(),
            ancestry: Vec::new(),
        })
    }

    /// Instantiate a core definition with the same validation and defaults as user rules.
    pub fn core_rule(
        &mut self,
        name: &str,
        inputs: BTreeMap<Str, Artifact>,
        parameters: BTreeMap<Str, ParameterValue>,
    ) -> BakeryResult<BTreeMap<Str, Artifact>> {
        let mut caller = Scope::new(self.repositories.core_repository);
        caller.inputs = inputs;
        self.invoke(
            &mut caller,
            &RuleInvocation {
                uses: name.into(),
                inputs: None,
                secrets: None,
                parameters: Some(
                    parameters
                        .into_iter()
                        .map(|(key, value)| (key, ParameterBinding::Value(value)))
                        .collect(),
                ),
            },
        )
    }

    /// Plan an invocation using the caller's available artifacts and parameters.
    #[tracing::instrument(skip_all)]
    pub fn plan(
        mut self,
        invocation: &RuleInvocation,
        inputs: BTreeMap<Str, Artifact>,
        parameters: BTreeMap<Str, ParameterValue>,
    ) -> BakeryResult<Plan> {
        let mut scope = Scope {
            repository: self.repositories.root_repository,
            inputs,
            parameters,
            ..Scope::new(self.repositories.root_repository)
        };
        let outputs = self.invoke(&mut scope, invocation)?;
        self.plan.outputs = outputs;
        Ok(self.plan)
    }

    /// Bind an invocation to its enclosing graph and expand its implementation.
    pub(crate) fn invoke(
        &mut self,
        caller: &mut Scope,
        invocation: &RuleInvocation,
    ) -> BakeryResult<BTreeMap<Str, Artifact>> {
        let (repository, name) = crate::project::catalog::resolve(
            &self.repositories,
            caller.repository,
            &invocation.uses,
        )?;
        let directory = self.repositories[repository]
            .source
            .dir
            .join("rules")
            .join(name);
        if self.ancestry.contains(&directory) {
            bail!("recursive rule composition involving {:?}", invocation.uses);
        }
        let definition: RuleDefinition = crate::config::load_config(&directory.join("rule.toml"))?;
        let mut scope = Scope::new(repository);
        let declared_inputs = definition.inputs.unwrap_or_default();
        reject_unknown(invocation.inputs.as_ref(), declared_inputs.keys(), "input")?;
        for (name, input) in declared_inputs {
            validate_name(&name)?;
            let artifact = match invocation
                .inputs
                .as_ref()
                .and_then(|inputs| inputs.get(&name))
            {
                Some(source) => self.artifact(caller, source, false)?,
                None => self.input(caller, &name).message(|| {
                    format!(
                        "while binding input {name:?} for rule {:?}",
                        invocation.uses
                    )
                })?,
            };
            if artifact.kind != input.kind() {
                bail!("input {name:?} has an incompatible artifact kind");
            }
            scope.inputs.insert(name, artifact);
        }
        let declared_parameters = definition.parameters.unwrap_or_default();
        reject_unknown(
            invocation.parameters.as_ref(),
            declared_parameters.keys(),
            "parameter",
        )?;
        for (name, definition) in declared_parameters {
            validate_name(&name)?;
            let value = match invocation
                .parameters
                .as_ref()
                .and_then(|parameters| parameters.get(&name))
            {
                Some(ParameterBinding::Value(value)) => value.clone(),
                Some(ParameterBinding::Reference(reference)) => caller
                    .parameters
                    .get(&reference.parameter)
                    .cloned()
                    .ok_or_else(|| whatever!("unknown parameter {:?}", reference.parameter))?,
                None => definition.default.ok_or_else(|| {
                    whatever!("missing parameter {name:?} for rule {:?}", invocation.uses)
                })?,
            };
            scope.parameters.insert(name, value);
        }
        let declared_secrets = definition.secrets.unwrap_or_default();
        reject_unknown(
            invocation.secrets.as_ref(),
            declared_secrets.iter(),
            "secret",
        )?;
        for name in declared_secrets {
            validate_name(&name)?;
            let source = invocation
                .secrets
                .as_ref()
                .and_then(|secrets| secrets.get(&name))
                .ok_or_else(|| {
                    whatever!("missing secret {name:?} for rule {:?}", invocation.uses)
                })?;
            scope
                .secrets
                .insert(name, self.artifact(caller, source, true)?);
        }
        self.ancestry.push(directory.clone());
        let outputs = match definition.execution {
            RuleExecution::Run(run) => {
                if run.command.is_empty() || run.outputs.is_empty() {
                    bail!("rule command and outputs must not be empty");
                }
                for (name, output) in &run.outputs {
                    validate_name(name)?;
                    validate_relative_path(Path::new(&output.path))?;
                }
                let env = run
                    .env
                    .as_ref()
                    .map(|env| {
                        self.envs
                            .lookup(repository, env)
                            .map(|env| env.context.clone())
                    })
                    .transpose()?;
                let task = self.plan.tasks.len();
                let outputs = run
                    .outputs
                    .iter()
                    .map(|(name, output)| {
                        (
                            name.clone(),
                            Artifact {
                                kind: output.kind,
                                source: ArtifactLocation::Output {
                                    task,
                                    name: name.clone(),
                                },
                            },
                        )
                    })
                    .collect();
                self.plan.tasks.push(Task::Run(RunTask {
                    name: invocation.uses.clone(),
                    directory,
                    env,
                    run,
                    inputs: scope.inputs,
                    parameters: scope.parameters,
                    secrets: scope.secrets,
                }));
                outputs
            }
            RuleExecution::Layer(layer) => {
                if !scope.secrets.is_empty() {
                    bail!("layer rules cannot bind secrets");
                }
                self.layer(
                    repository,
                    &layer.layer,
                    layer.architecture,
                    scope.inputs,
                    scope.parameters,
                )?
            }
            RuleExecution::Graph(graph) => {
                if graph.outputs.is_empty() {
                    bail!("composition must export at least one output");
                }
                for name in graph.rules.keys().chain(graph.outputs.keys()) {
                    validate_name(name)?;
                }
                let sources = graph.outputs.clone();
                scope.graph = Some(graph);
                let mut outputs = BTreeMap::new();
                for (name, source) in sources {
                    outputs.insert(name, self.artifact(&mut scope, &source, false)?);
                }
                outputs
            }
        };
        self.ancestry.pop();
        Ok(outputs)
    }

    /// Resolve an artifact, planning dependencies on demand.
    pub(crate) fn artifact(
        &mut self,
        scope: &mut Scope,
        source: &ArtifactSource,
        secret: bool,
    ) -> BakeryResult<Artifact> {
        match source {
            ArtifactSource::Select(selection) => {
                validate_relative_path(Path::new(&selection.path))?;
                let source = self.artifact(scope, &selection.source, secret)?;
                if source.kind != ArtifactKind::Directory {
                    bail!("artifact selection requires a directory");
                }
                Ok(Artifact {
                    kind: selection.kind,
                    source: ArtifactLocation::Select {
                        source: Box::new(source),
                        path: selection.path.as_str().into(),
                    },
                })
            }
            ArtifactSource::Tree(entries) => {
                if secret {
                    bail!("secret bindings cannot assemble artifact trees");
                }
                let mut tree = BTreeMap::<PathBuf, Artifact>::new();
                for (path, source) in entries {
                    let path = Path::new(path.as_str());
                    validate_relative_path(path)?;
                    if tree
                        .keys()
                        .any(|other| path.starts_with(other) || other.starts_with(path))
                    {
                        bail!("overlapping artifact tree destination {path:?}");
                    }
                    tree.insert(path.to_owned(), self.artifact(scope, source, false)?);
                }
                Ok(Artifact {
                    kind: ArtifactKind::Directory,
                    source: ArtifactLocation::Tree(tree),
                })
            }
            ArtifactSource::Input(name) => {
                if secret {
                    scope
                        .secrets
                        .get(name)
                        .cloned()
                        .ok_or_else(|| whatever!("unknown secret {name:?}"))
                } else {
                    self.input(scope, name)
                }
            }
            ArtifactSource::Path(path) => {
                Artifact::from_path(&self.repositories[scope.repository].source.dir.join(path))
            }
            ArtifactSource::Rule(reference) => {
                if !scope.completed.contains_key(&reference.node) {
                    if !scope.visiting.insert(reference.node.clone()) {
                        bail!("build dependency cycle involving {:?}", reference.node);
                    }
                    let invocation = scope
                        .graph
                        .as_ref()
                        .and_then(|graph| graph.rules.get(&reference.node))
                        .cloned()
                        .ok_or_else(|| whatever!("unknown graph node {:?}", reference.node))?;
                    let outputs = self.invoke(scope, &invocation)?;
                    scope.completed.insert(reference.node.clone(), outputs);
                    scope.visiting.remove(&reference.node);
                }
                scope.completed[&reference.node]
                    .get(&reference.output)
                    .cloned()
                    .ok_or_else(|| {
                        whatever!(
                            "unknown output {:?} of node {:?}",
                            reference.output,
                            reference.node
                        )
                    })
            }
        }
    }

    /// Resolve recipe defaults lazily, including dependencies between their inputs.
    pub(crate) fn input(&mut self, scope: &mut Scope, name: &Str) -> BakeryResult<Artifact> {
        if let Some(artifact) = scope.inputs.get(name) {
            return Ok(artifact.clone());
        }
        let source = scope
            .input_defaults
            .get(name)
            .cloned()
            .ok_or_else(|| whatever!("missing input {name:?}"))?;
        if !scope.input_visiting.insert(name.clone()) {
            bail!("build dependency cycle involving input {name:?}");
        }
        let artifact = self.artifact(scope, &source, false)?;
        scope.input_visiting.remove(name);
        scope.inputs.insert(name.clone(), artifact.clone());
        Ok(artifact)
    }
}

/// Local bindings and memoized dependency expansions for one composition.
pub(crate) struct Scope {
    pub(crate) repository: RepositoryIdx,
    pub(crate) inputs: BTreeMap<Str, Artifact>,
    pub(crate) input_defaults: Map<Str, ArtifactSource>,
    input_visiting: HashSet<Str>,
    pub(crate) parameters: BTreeMap<Str, ParameterValue>,
    secrets: BTreeMap<Str, Artifact>,
    pub(crate) graph: Option<GraphRule>,
    completed: HashMap<Str, BTreeMap<Str, Artifact>>,
    visiting: HashSet<Str>,
}

/// Reject misspelled bindings instead of silently ignoring configuration.
fn reject_unknown<'a, T>(
    supplied: Option<&Map<Str, T>>,
    declared: impl Iterator<Item = &'a Str>,
    kind: &str,
) -> BakeryResult<()> {
    let declared: HashSet<_> = declared.collect();
    if let Some(supplied) = supplied {
        for name in supplied.keys() {
            if !declared.contains(name) {
                bail!("unknown {kind} binding {name:?}");
            }
        }
    }
    Ok(())
}

pub use crate::project::catalog::validate_name;

/// Output paths must stay within the command's private output directory.
pub fn validate_relative_path(path: &Path) -> BakeryResult<()> {
    if path.as_os_str().is_empty()
        || !path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        bail!("output path must be a nonempty relative path without '..'");
    }
    Ok(())
}

impl Scope {
    /// Keep prerequisite names local to their layer or recipe.
    pub(crate) fn with_rules(
        repository: RepositoryIdx,
        rules: Option<&Map<Str, RuleInvocation>>,
    ) -> BakeryResult<Self> {
        let mut scope = Self::new(repository);
        if let Some(rules) = rules {
            for name in rules.keys() {
                validate_name(name)?;
            }
            scope.graph = Some(GraphRule {
                rules: rules.clone(),
                outputs: Map::new(),
            });
        }
        Ok(scope)
    }

    pub(crate) fn new(repository: RepositoryIdx) -> Self {
        Self {
            repository,
            inputs: BTreeMap::new(),
            input_defaults: Map::new(),
            input_visiting: HashSet::new(),
            parameters: BTreeMap::new(),
            secrets: BTreeMap::new(),
            graph: None,
            completed: HashMap::new(),
            visiting: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use std::fs;
    use std::path::Path;
    use std::sync::Arc;

    use super::Task;
    use crate::config::repositories::{PathSourceConfig, RepositoryConfig, SourceConfig};
    use crate::config::rules::RuleInvocation;
    use crate::project::repositories::{ProjectRepositories, Repository, RepositoryIdx, Source};
    use crate::utils::idx_vec::{Idx, IdxVec};

    use super::{Artifact, Planner};

    fn repositories(root: &Path) -> Arc<ProjectRepositories> {
        let mut repos = Vec::new();
        for (index, name) in ["project", "vendor", "tooling"].iter().enumerate() {
            let dir = root.join(name);
            fs::create_dir_all(&dir).unwrap();
            let idx = RepositoryIdx::from_usize(index);
            let source = Source::materialize(
                SourceConfig::Path(PathSourceConfig {
                    path: dir.to_string_lossy().into_owned().into(),
                }),
                root,
                false,
            )
            .unwrap();
            let dependencies = match index {
                0 => HashMap::from([("vendor".to_owned(), RepositoryIdx::from_usize(1))]),
                1 => HashMap::from([("tools".to_owned(), RepositoryIdx::from_usize(2))]),
                _ => HashMap::new(),
            };
            repos.push(Repository {
                idx,
                source,
                repositories: dependencies,
                config: RepositoryConfig {
                    name: None,
                    description: None,
                    repositories: None,
                },
            });
        }
        Arc::new(ProjectRepositories {
            repositories: IdxVec::from_vec(repos),
            root_repository: RepositoryIdx::from_usize(0),
            core_repository: RepositoryIdx::from_usize(2),
        })
    }

    fn write(root: &Path, path: &str, contents: &str) {
        let path = root.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    /// Imported compositions resolve their own dependencies and share predecessor nodes.
    #[test]
    fn compose_imported_rules_with_shared_artifacts() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        let repositories = repositories(root);
        write(root, "tooling/envs/runner/Dockerfile", "FROM scratch\n");
        write(
            root,
            "tooling/rules/copy/rule.toml",
            r#"
[inputs]
source = "file"
[execution]
type = "run"
env = "runner"
command = ["cp", "/run/rugix/bakery/work/inputs/source", "/run/rugix/bakery/work/outputs/result"]
[execution.outputs.result]
kind = "file"
path = "result"
"#,
        );
        write(
            root,
            "vendor/rules/pair/rule.toml",
            r#"
[inputs]
source = "file"
[execution]
type = "graph"
[execution.rules.copy]
uses = "tools/copy"
[execution.outputs]
first = { rule = { node = "copy", output = "result" } }
second = { rule = { node = "copy", output = "result" } }
"#,
        );
        write(root, "source", "payload");
        let invocation: RuleInvocation =
            crate::config::parse_config("uses = \"vendor/pair\"").unwrap();
        let plan = Planner::new(repositories)
            .unwrap()
            .plan(
                &invocation,
                BTreeMap::from([(
                    "source".into(),
                    Artifact::from_path(&root.join("source")).unwrap(),
                )]),
                BTreeMap::new(),
            )
            .unwrap();
        assert_eq!(plan.tasks.len(), 1);
        assert_eq!(plan.outputs.len(), 2);
        let Task::Run(task) = &plan.tasks[0] else {
            panic!("expected run task")
        };
        assert_eq!(
            task.env,
            Some(root.join("tooling/envs/runner").canonicalize().unwrap())
        );
        assert_eq!(task.directory, root.join("tooling/rules/copy"));
    }

    /// A cycle between node inputs fails while planning, before environment construction.
    #[test]
    fn reject_dependency_cycles() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        let repositories = repositories(root);
        write(
            root,
            "vendor/rules/cycle/rule.toml",
            r#"
[execution]
type = "graph"
[execution.rules.left]
uses = "copy"
inputs.source = { rule = { node = "right", output = "result" } }
[execution.rules.right]
uses = "copy"
inputs.source = { rule = { node = "left", output = "result" } }
[execution.outputs]
result = { rule = { node = "left", output = "result" } }
"#,
        );
        write(
            root,
            "vendor/rules/copy/rule.toml",
            r#"
[inputs]
source = "file"
[execution]
type = "run"
env = "unused"
command = ["true"]
[execution.outputs.result]
kind = "file"
path = "result"
"#,
        );
        let invocation: RuleInvocation =
            crate::config::parse_config("uses = \"vendor/cycle\"").unwrap();
        let error = Planner::new(repositories)
            .unwrap()
            .plan(&invocation, BTreeMap::new(), BTreeMap::new())
            .unwrap_err();
        assert!(format!("{error:?}").contains("dependency cycle"));
    }

    /// Interface mistakes fail before running a command or requiring its environment.
    #[test]
    fn reject_incompatible_or_unknown_inputs() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        let repositories = repositories(root);
        write(
            root,
            "vendor/rules/copy/rule.toml",
            r#"
[inputs]
source = "file"
[execution]
type = "run"
env = "unused"
command = ["true"]
[execution.outputs.result]
kind = "file"
path = "../escape"
"#,
        );
        for invocation in [
            "uses = \"vendor/copy\"\ninputs.typo = { path = \".\" }",
            "uses = \"vendor/copy\"\ninputs.source = { path = \".\" }",
        ] {
            let invocation = crate::config::parse_config(invocation).unwrap();
            assert!(Planner::new(repositories.clone())
                .unwrap()
                .plan(&invocation, BTreeMap::new(), BTreeMap::new())
                .is_err());
        }
    }
}
