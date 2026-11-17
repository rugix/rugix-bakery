//! Rule parameters: replaceable graph operations with fixed interfaces.
//!
//! A graph declares each rule parameter with an interface and a default
//! implementation. Callers bind another rule, add bindings to the default, or
//! forward a rule parameter of their own graph. Bindings resolve in the scope that
//! contains them, so an implementation carries its additional inputs, parameters,
//! and secrets to every call site. Call sites supply exactly the interface inputs and
//! parameters, and the planner checks the implementation's outputs against the
//! interface before the build starts.
//!
//! Implementations expand into ordinary tasks. The cache identity of each task covers
//! its implementation, environment, and bindings, so replacing an implementation or
//! changing its bindings invalidates exactly the affected outputs.

use std::collections::BTreeMap;

use reportify::{bail, whatever, ResultExt};
use sidex::types::{Map, Str};

use crate::config::recipes::ParameterValue;
use crate::config::rules::{
    ArtifactSource, ParameterBinding, RuleBinding, RuleInvocation, RuleParameterDef, RuleReference,
};
use crate::project::repositories::RepositoryIdx;
use crate::BakeryResult;

use super::planner::{reject_unknown, validate_name, Artifact, Planner, Scope};

/// Resolved bindings for one rule instantiation.
#[derive(Clone, Debug, Default)]
pub(crate) struct Bindings {
    pub(crate) inputs: BTreeMap<Str, Artifact>,
    pub(crate) parameters: BTreeMap<Str, ParameterValue>,
    pub(crate) secrets: BTreeMap<Str, Artifact>,
    pub(crate) rules: BTreeMap<Str, Selection>,
}

/// A caller's choice for one rule parameter.
#[derive(Clone, Debug)]
pub(crate) enum Selection {
    /// Keep the default implementation and add these bindings.
    Default(Bindings),
    /// Replace the default implementation.
    Replace(Implementation),
}

/// A rule reference with bindings resolved in the scope that named it.
#[derive(Clone, Debug)]
pub(crate) struct Implementation {
    /// Repository relative to which `uses` resolves.
    pub(crate) repository: RepositoryIdx,
    pub(crate) uses: Str,
    pub(crate) bindings: Bindings,
}

/// A declared rule parameter and its selected implementation.
#[derive(Clone, Debug)]
pub(crate) struct RuleParameter {
    pub(crate) interface: RuleParameterDef,
    pub(crate) implementation: Implementation,
}

/// Unresolved bindings of an invocation or rule parameter binding.
pub(crate) struct BindingSources<'a> {
    inputs: Option<&'a Map<Str, ArtifactSource>>,
    parameters: Option<&'a Map<Str, ParameterBinding>>,
    secrets: Option<&'a Map<Str, ArtifactSource>>,
    rules: Option<&'a Map<Str, RuleBinding>>,
}

impl Planner {
    /// Resolve bindings in `scope`, planning referenced graph nodes on demand.
    pub(crate) fn bind(
        &mut self,
        scope: &mut Scope,
        sources: BindingSources<'_>,
    ) -> BakeryResult<Bindings> {
        let mut bindings = Bindings::default();
        for (name, source) in sources.inputs.into_iter().flatten() {
            validate_name(name)?;
            let artifact = self.artifact(scope, source, false)?;
            bindings.inputs.insert(name.clone(), artifact);
        }
        for (name, binding) in sources.parameters.into_iter().flatten() {
            validate_name(name)?;
            let value = match binding {
                ParameterBinding::Value(value) => value.clone(),
                ParameterBinding::Reference(reference) => scope
                    .parameters
                    .get(&reference.parameter)
                    .cloned()
                    .ok_or_else(|| whatever!("unknown parameter {:?}", reference.parameter))?,
            };
            bindings.parameters.insert(name.clone(), value);
        }
        for (name, source) in sources.secrets.into_iter().flatten() {
            validate_name(name)?;
            let artifact = self.artifact(scope, source, true)?;
            bindings.secrets.insert(name.clone(), artifact);
        }
        for (name, binding) in sources.rules.into_iter().flatten() {
            validate_name(name)?;
            let nested = self.bind(scope, binding.into())?;
            let selection = match &binding.uses {
                None => Selection::Default(nested),
                Some(RuleReference::Rule(uses)) => Selection::Replace(Implementation {
                    repository: scope.repository,
                    uses: uses.clone(),
                    bindings: nested,
                }),
                Some(RuleReference::Parameter(reference)) => {
                    if !nested.is_empty() {
                        bail!(
                            "binding {name:?} forwards rule parameter {:?} and cannot add bindings",
                            reference.rule_parameter
                        );
                    }
                    let forwarded = scope
                        .rule_parameters
                        .get(&reference.rule_parameter)
                        .ok_or_else(|| {
                            whatever!("unknown rule parameter {:?}", reference.rule_parameter)
                        })?;
                    Selection::Replace(forwarded.implementation.clone())
                }
            };
            bindings.rules.insert(name.clone(), selection);
        }
        Ok(bindings)
    }

    /// Select the implementation of each declared rule parameter of a graph.
    ///
    /// Defaults resolve in the graph's own scope, so they may reference its inputs,
    /// parameters, and secrets.
    pub(crate) fn select_rule_parameters(
        &mut self,
        scope: &mut Scope,
        declared: Map<Str, RuleParameterDef>,
        mut selections: BTreeMap<Str, Selection>,
    ) -> BakeryResult<()> {
        for (name, interface) in declared {
            validate_name(&name)?;
            if interface.outputs.is_empty() {
                bail!("rule parameter {name:?} must declare at least one output");
            }
            let implementation = match selections.remove(&name) {
                Some(Selection::Replace(implementation)) => implementation,
                selection => {
                    let RuleReference::Rule(uses) = &interface.default.uses else {
                        bail!("default of rule parameter {name:?} must name a rule");
                    };
                    let mut bindings = self.bind(scope, (&interface.default).into())?;
                    if let Some(Selection::Default(additional)) = selection {
                        bindings.extend(additional).message(|| {
                            format!("while adding bindings to the default of {name:?}")
                        })?;
                    }
                    Implementation {
                        repository: scope.repository,
                        uses: uses.clone(),
                        bindings,
                    }
                }
            };
            let supplied_by_graph = interface
                .inputs
                .iter()
                .flat_map(|inputs| inputs.keys())
                .filter(|input| implementation.bindings.inputs.contains_key(*input))
                .chain(
                    interface
                        .parameters
                        .iter()
                        .flat_map(|parameters| parameters.keys())
                        .filter(|parameter| {
                            implementation.bindings.parameters.contains_key(*parameter)
                        }),
                )
                .next();
            if let Some(binding) = supplied_by_graph {
                bail!(
                    "binding {binding:?} of rule parameter {name:?} is supplied by the graph at each call"
                );
            }
            scope.rule_parameters.insert(
                name,
                RuleParameter {
                    interface,
                    implementation,
                },
            );
        }
        Ok(())
    }

    /// Expand the selected implementation of a rule parameter at one call site.
    pub(crate) fn call_rule_parameter(
        &mut self,
        caller: &mut Scope,
        name: &Str,
        mut call: Bindings,
    ) -> BakeryResult<BTreeMap<Str, Artifact>> {
        let RuleParameter {
            interface,
            mut implementation,
        } = caller
            .rule_parameters
            .get(name)
            .cloned()
            .ok_or_else(|| whatever!("unknown rule parameter {name:?}"))?;
        if !call.secrets.is_empty() || !call.rules.is_empty() {
            bail!("calls of rule parameter {name:?} bind only interface inputs and parameters");
        }
        let inputs = interface.inputs.unwrap_or_default();
        reject_unknown(call.inputs.keys(), inputs.keys(), "input")?;
        for (input, declaration) in inputs {
            let artifact = call
                .inputs
                .remove(&input)
                .ok_or_else(|| whatever!("missing input {input:?} for rule parameter {name:?}"))?;
            if artifact.kind != declaration.kind {
                bail!(
                    "input {input:?} of rule parameter {name:?} has an incompatible artifact kind"
                );
            }
            implementation.bindings.inputs.insert(input, artifact);
        }
        let parameters = interface.parameters.unwrap_or_default();
        reject_unknown(call.parameters.keys(), parameters.keys(), "parameter")?;
        for parameter in parameters.into_keys() {
            let value = call.parameters.remove(&parameter).ok_or_else(|| {
                whatever!("missing parameter {parameter:?} for rule parameter {name:?}")
            })?;
            implementation.bindings.parameters.insert(parameter, value);
        }
        let uses = implementation.uses.clone();
        let mut outputs = self
            .instantiate(implementation, None)
            .message(|| format!("while planning {uses:?} for rule parameter {name:?}"))?;
        interface
            .outputs
            .into_iter()
            .map(|(output, declaration)| {
                let artifact = outputs.remove(&output).ok_or_else(|| {
                    whatever!(
                        "implementation {uses:?} of rule parameter {name:?} has no output {output:?}"
                    )
                })?;
                if artifact.kind != declaration.kind {
                    bail!(
                        "output {output:?} of {uses:?} for rule parameter {name:?} has an incompatible artifact kind"
                    );
                }
                Ok((output, artifact))
            })
            .collect()
    }
}

impl Bindings {
    fn is_empty(&self) -> bool {
        self.inputs.is_empty()
            && self.parameters.is_empty()
            && self.secrets.is_empty()
            && self.rules.is_empty()
    }

    /// Add bindings from another scope, rejecting names bound twice.
    fn extend(&mut self, other: Bindings) -> BakeryResult<()> {
        fn merge<T>(target: &mut BTreeMap<Str, T>, source: BTreeMap<Str, T>) -> BakeryResult<()> {
            for (name, value) in source {
                if target.insert(name.clone(), value).is_some() {
                    bail!("{name:?} is already bound");
                }
            }
            Ok(())
        }
        merge(&mut self.inputs, other.inputs)?;
        merge(&mut self.parameters, other.parameters)?;
        merge(&mut self.secrets, other.secrets)?;
        merge(&mut self.rules, other.rules)
    }
}

impl<'a> From<&'a RuleInvocation> for BindingSources<'a> {
    fn from(invocation: &'a RuleInvocation) -> Self {
        Self {
            inputs: invocation.inputs.as_ref(),
            parameters: invocation.parameters.as_ref(),
            secrets: invocation.secrets.as_ref(),
            rules: invocation.rules.as_ref(),
        }
    }
}

impl<'a> From<&'a RuleBinding> for BindingSources<'a> {
    fn from(binding: &'a RuleBinding) -> Self {
        Self {
            inputs: binding.inputs.as_ref(),
            parameters: binding.parameters.as_ref(),
            secrets: binding.secrets.as_ref(),
            rules: binding.rules.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::Path;

    use sidex::types::Str;

    use crate::build::planner::tests::{repositories, write};
    use crate::build::planner::{Artifact, ArtifactLocation, Plan, Planner, RunTask, Task};
    use crate::config::recipes::ParameterValue;
    use crate::config::rules::RuleInvocation;
    use crate::BakeryResult;

    const SIGNER_INTERFACE: &str = r#"
[rule-parameters.signer]
inputs.image = { kind = "file" }
inputs.certificate = { kind = "file" }
parameters.signing-time = {}
outputs.image = { kind = "file" }
default = { uses = "sign" }
"#;

    /// A run rule accepting the signer interface with a configurable output kind.
    fn signer(kind: &str) -> String {
        format!(
            r#"
[inputs.image]
kind = "file"
[inputs.certificate]
kind = "file"
[parameters.signing-time]
[execution]
type = "run"
command = ["true"]
[execution.outputs.image]
kind = "{kind}"
path = "signed"
"#
        )
    }

    /// Write a graph that signs twice, a wrapper forwarding its signer, and signers.
    fn project(root: &Path) {
        write(
            root,
            "vendor/rules/boot/rule.toml",
            &format!(
                r#"
[inputs]
image = {{ kind = "file" }}
certificate = {{ kind = "file" }}
{SIGNER_INTERFACE}
[execution]
type = "graph"
[execution.rules.first]
uses = {{ rule-parameter = "signer" }}
inputs.image = {{ input = "image" }}
inputs.certificate = {{ input = "certificate" }}
parameters.signing-time = "1"
[execution.rules.second]
uses = {{ rule-parameter = "signer" }}
inputs.image = {{ rule = {{ node = "first", output = "image" }} }}
inputs.certificate = {{ input = "certificate" }}
parameters.signing-time = "2"
[execution.outputs]
image = {{ rule = {{ node = "second", output = "image" }} }}
"#
            ),
        );
        write(
            root,
            "vendor/rules/wrapper/rule.toml",
            &format!(
                r#"
[inputs]
image = {{ kind = "file" }}
certificate = {{ kind = "file" }}
{SIGNER_INTERFACE}
[execution]
type = "graph"
[execution.rules.boot]
uses = "boot"
rules.signer = {{ uses = {{ rule-parameter = "signer" }} }}
[execution.outputs]
image = {{ rule = {{ node = "boot", output = "image" }} }}
"#
            ),
        );
        write(
            root,
            "vendor/rules/sign/rule.toml",
            &format!("secrets = [\"key\"]\n{}", signer("file")),
        );
        write(
            root,
            "project/rules/hsm-sign/rule.toml",
            &format!(
                "secrets = [\"pin\"]\n[inputs.token]\nkind = \"directory\"\n[parameters.slot]\ndefault = 0\n{}",
                signer("file").replace("type = \"run\"", "type = \"run\"\nenv = \"hsm\"")
            ),
        );
        write(root, "project/envs/hsm/Dockerfile", "FROM scratch\n");
        write(
            root,
            "project/rules/directory-image/rule.toml",
            &signer("directory"),
        );
        write(
            root,
            "project/rules/narrow/rule.toml",
            r#"
[inputs.image]
kind = "file"
[execution]
type = "run"
command = ["true"]
[execution.outputs.image]
kind = "file"
path = "signed"
"#,
        );
        for path in ["image", "certificate", "key", "pin", "token/state"] {
            write(root, &format!("project/{path}"), "content");
        }
    }

    /// Plan the wrapper with the given rule parameter bindings.
    fn plan(root: &Path, bindings: &str) -> BakeryResult<Plan> {
        let invocation: RuleInvocation =
            crate::config::parse_config(&format!("uses = \"vendor/wrapper\"\n{bindings}")).unwrap();
        let inputs = ["image", "certificate"]
            .into_iter()
            .map(|name| {
                (
                    Str::from(name),
                    Artifact::from_path(&root.join("project").join(name)).unwrap(),
                )
            })
            .collect();
        Planner::new(repositories(root))
            .unwrap()
            .plan(&invocation, inputs, BTreeMap::new())
    }

    fn run_tasks(plan: &Plan) -> Vec<&RunTask> {
        plan.tasks
            .iter()
            .map(|task| match task {
                Task::Run(task) => task,
                _ => panic!("expected run task"),
            })
            .collect()
    }

    /// Whether an artifact is the given file or directory of the project repository.
    fn is_project_path(artifact: &Artifact, root: &Path, path: &str) -> bool {
        matches!(
            &artifact.source,
            ArtifactLocation::Path(source)
                if *source == root.join("project").join(path).canonicalize().unwrap()
        )
    }

    /// Defaults and replacements receive interface bindings at each call site and
    /// their own bindings from the caller, also through a forwarding wrapper.
    #[test]
    fn bind_rule_parameter_implementations() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        project(root);

        let default = plan(root, "rules.signer.secrets.key = { path = \"key\" }").unwrap();
        let tasks = run_tasks(&default);
        assert_eq!(tasks.len(), 2);
        for (task, time) in tasks.iter().zip(["1", "2"]) {
            assert_eq!(task.directory, root.join("vendor/rules/sign"));
            assert!(is_project_path(&task.secrets["key"], root, "key"));
            assert!(matches!(
                &task.parameters["signing-time"],
                ParameterValue::String(value) if value.as_str() == time
            ));
        }
        assert!(is_project_path(&tasks[0].inputs["image"], root, "image"));
        assert!(matches!(
            tasks[1].inputs["image"].source,
            ArtifactLocation::Output { task: 0, .. }
        ));

        let replaced = plan(
            root,
            r#"
[rules.signer]
uses = "hsm-sign"
inputs.token = { path = "token" }
parameters.slot = 3
secrets.pin = { path = "pin" }
"#,
        )
        .unwrap();
        let tasks = run_tasks(&replaced);
        assert_eq!(tasks.len(), 2);
        for task in tasks {
            assert_eq!(task.directory, root.join("project/rules/hsm-sign"));
            assert_eq!(
                task.env,
                Some(root.join("project/envs/hsm").canonicalize().unwrap())
            );
            assert!(is_project_path(&task.inputs["token"], root, "token"));
            assert!(matches!(
                task.parameters["slot"],
                ParameterValue::Integer(3)
            ));
            assert!(is_project_path(&task.secrets["pin"], root, "pin"));
        }
        assert!(matches!(
            replaced.outputs["image"].source,
            ArtifactLocation::Output { task: 1, .. }
        ));
    }

    /// Incompatible implementations and bindings fail while planning.
    #[test]
    fn reject_incompatible_rule_parameter_bindings() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        project(root);
        for (bindings, expected) in [
            ("", "missing secret \"key\""),
            (
                "rules.typo = { uses = \"narrow\" }",
                "unknown rule parameter binding",
            ),
            (
                "rules.signer = { uses = { rule-parameter = \"signer\" } }",
                "unknown rule parameter \"signer\"",
            ),
            (
                "rules.signer = { uses = \"narrow\" }",
                "unknown input binding",
            ),
            (
                "rules.signer = { uses = \"directory-image\" }",
                "incompatible artifact kind",
            ),
            (
                "rules.signer = { uses = \"hsm-sign\", secrets.pin = { path = \"pin\" } }",
                "missing input \"token\"",
            ),
            (
                "rules.signer = { secrets.key = { path = \"key\" }, inputs.image = { path = \"image\" } }",
                "supplied by the graph",
            ),
        ] {
            let error = plan(root, bindings).unwrap_err();
            assert!(
                format!("{error:?}").contains(expected),
                "{bindings}: {error:?}"
            );
        }
    }
}
