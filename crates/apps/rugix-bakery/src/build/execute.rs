//! Rule execution, atomic result caching, and declared output validation.

use std::collections::{BTreeMap, HashMap};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use reportify::{bail, ResultExt};
use sha2::{Digest, Sha256};
use sidex::types::Str;
use tempfile::TempDir;
use tracing::info;

use crate::config::envs::EnvStorage;
use crate::config::rules::ArtifactKind;
use crate::envs::{BindMount, BuiltEnv, EnvStore};
use crate::project::envs::EnvDefinition;
use crate::BakeryResult;

use super::artifacts::Artifacts;
use super::cache::artifact_hash;
use super::dependencies::ProjectDependencies;
use super::secrets::SecretFingerprints;
use super::{runtime, Plan, RunTask, Task};

/// One build session. Uncached outputs remain alive until publication.
pub struct Executor {
    project: PathBuf,
    cache: PathBuf,
    env_cache: PathBuf,
    images: HashMap<PathBuf, BuiltEnv>,
    storage: EnvStorage,
    temporary: Vec<TempDir>,
    artifacts: Artifacts,
}

impl Executor {
    pub fn new(
        project: &Path,
        cache: &Path,
        env_cache: &Path,
        storage: EnvStorage,
    ) -> BakeryResult<Self> {
        fs::create_dir_all(cache).whatever("unable to create rule cache")?;
        Ok(Self {
            project: fs::canonicalize(project).whatever("unable to resolve project directory")?,
            cache: fs::canonicalize(cache).whatever("unable to resolve rule cache")?,
            env_cache: env_cache.to_owned(),
            images: HashMap::new(),
            storage,
            temporary: Vec::new(),
            artifacts: Artifacts::new(cache)?,
        })
    }

    /// Use the standard project-owned result cache and shared tooling environment store.
    pub fn for_project(project: &crate::project::ProjectRef) -> BakeryResult<Self> {
        let env_cache = Path::new("/run/rugix/bakery/cache")
            .join(project.local_id().as_str())
            .join("envs");
        Self::new(
            project.dir(),
            &project.dir().join(".rugix/rules"),
            &env_cache,
            EnvStorage::Auto,
        )
    }

    /// Preserve a standalone artifact whose path must outlive this session.
    pub fn keep_artifact(&mut self, artifact: &Path) {
        if let Some(index) = self
            .temporary
            .iter()
            .position(|directory| artifact.starts_with(directory.path()))
        {
            let _ = self.temporary.swap_remove(index).keep();
        }
    }

    /// Execute a validated plan in dependency order.
    #[tracing::instrument(skip_all)]
    pub fn execute(&mut self, plan: &Plan) -> BakeryResult<BTreeMap<Str, PathBuf>> {
        let lock = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(self.cache.join("build.lock"))
            .whatever("unable to open build lock")?;
        lock.lock().whatever("unable to lock rule cache")?;
        let bakery_identity = if plan
            .tasks
            .iter()
            .any(|task| !matches!(task, Task::Run(task) if task.env.is_some()))
        {
            runtime::identity()?
        } else {
            None
        };
        let project_mounts = runtime::project_mounts(&self.project)?;
        self.execute_plan(plan, bakery_identity.as_deref(), &project_mounts)
    }

    /// Expanded graphs share the outer lock, artifact identities, environments, and
    /// temporaries.
    fn execute_plan(
        &mut self,
        plan: &Plan,
        bakery_identity: Option<&str>,
        project_mounts: &[BindMount],
    ) -> BakeryResult<BTreeMap<Str, PathBuf>> {
        let mut results = Vec::new();
        for task in &plan.tasks {
            rugix_tasks::check_canceled();
            let outputs = match task {
                Task::Graph(task) => {
                    let inputs = self.artifacts.resolve_all(&task.inputs, &results)?;
                    let expanded = (task.expand)(&inputs)?;
                    self.execute_plan(&expanded, bakery_identity, project_mounts)?
                }
                Task::Run(task) => {
                    self.execute_run(task, &results, bakery_identity, project_mounts)?
                }
                Task::Layer(task) => {
                    let inputs = self.artifacts.resolve_all(&task.inputs, &results)?;
                    let mut hashes = BTreeMap::new();
                    for (name, path) in &inputs {
                        hashes.insert(name.clone(), self.artifacts.hash(path)?);
                    }
                    let temporary = if bakery_identity.is_none() {
                        let directory = self.project.join(".rugix/layers");
                        fs::create_dir_all(&directory)
                            .whatever("unable to create layer workspace directory")?;
                        Some(
                            tempfile::tempdir_in(directory)
                                .whatever("unable to create layer workspace")?,
                        )
                    } else {
                        None
                    };
                    let workspace = temporary.as_ref().map(|directory| {
                        directory
                            .path()
                            .strip_prefix(&self.project)
                            .expect("layer workspace is inside the project")
                    });
                    let outputs = task.execute(&inputs, &hashes, bakery_identity, workspace)?;
                    if let Some(temporary) = temporary {
                        self.temporary.push(temporary);
                    }
                    outputs
                }
                Task::Release(task) => {
                    let inputs = self.artifacts.resolve_all(
                        &BTreeMap::from([("archive".into(), task.archive.clone())]),
                        &results,
                    )?;
                    let temporary = tempfile::tempdir_in(&self.cache)
                        .whatever("unable to create release workspace")?;
                    let outputs = task.execute(&inputs["archive"], temporary.path())?;
                    self.temporary.push(temporary);
                    outputs
                }
            };
            results.push(outputs);
        }
        self.artifacts.resolve_all(&plan.outputs, &results)
    }
    /// Execute one command after its declared graph predecessors have completed.
    fn execute_run(
        &mut self,
        task: &RunTask,
        results: &[BTreeMap<Str, PathBuf>],
        bakery_identity: Option<&str>,
        project_mounts: &[BindMount],
    ) -> BakeryResult<BTreeMap<Str, PathBuf>> {
        let inputs = self.artifacts.resolve_all(&task.inputs, results)?;
        let secrets = self.artifacts.resolve_all(&task.secrets, results)?;
        let identity = if let Some(env) = &task.env {
            if !self.images.contains_key(env) {
                let store = EnvStore::new(&self.env_cache, self.storage)?;
                let image = store.build(
                    &EnvDefinition {
                        context: env.clone(),
                    },
                    false,
                )?;
                self.images.insert(env.clone(), image);
            }
            Some(format!("env:{}", self.images[env].info().image_id))
        } else {
            bakery_identity.map(|id| format!("bakery:{id}"))
        };
        let mut key = Sha256::new();
        key.update(b"rugix-artifact-build-v4\0");
        push(
            &mut key,
            "runtime",
            identity.as_deref().unwrap_or("unidentified-bakery"),
        );
        let directory = fs::canonicalize(&task.directory)
            .whatever("unable to resolve rule implementation directory")?;
        push(&mut key, "implementation", &artifact_hash(&directory)?);
        push(
            &mut key,
            "parameters",
            &serde_json::to_string(&task.parameters)
                .whatever("unable to encode rule parameters")?,
        );
        for (name, path) in &inputs {
            let hash = self.artifacts.hash(path)?;
            push(&mut key, name, &hash);
        }
        let cached = task.run.cache.unwrap_or(true) && identity.is_some();
        if cached && !secrets.is_empty() {
            let fingerprints = SecretFingerprints::open(&self.cache)?;
            for (name, path) in &secrets {
                push(
                    &mut key,
                    &format!("secret:{name}"),
                    &fingerprints.fingerprint(path)?,
                );
            }
        }
        let key = hex::encode(key.finalize());
        let dependency_record = self.cache.join("dependencies").join(format!("{key}.json"));
        let reusable = if cached {
            match ProjectDependencies::load(&dependency_record)? {
                Some(dependencies) if dependencies.is_current(&self.project)? => {
                    let destination = self.cache.join(dependencies.cache_key(&key)?);
                    destination.is_dir().then_some(destination)
                }
                _ => None,
            }
        } else {
            None
        };
        let outputs = if let Some(destination) = reusable {
            info!("Reusing rule {:?}.", task.name);
            validate_outputs(task, &destination)?
        } else {
            info!("Building rule {:?}.", task.name);
            let temporary = tempfile::Builder::new()
                .prefix("rule-")
                .tempdir_in(&self.cache)
                .whatever("unable to create rule workspace")?;
            let output_dir = temporary.path().join("outputs");
            fs::create_dir(&output_dir).whatever("unable to create rule output directory")?;
            let parameter_file = temporary.path().join("parameters.json");
            fs::write(
                &parameter_file,
                serde_json::to_vec(&task.parameters)
                    .whatever("unable to encode rule parameters")?,
            )
            .whatever("unable to write rule parameters")?;
            let registrations = temporary.path().join("rebuild-if-changed.txt");
            fs::write(&registrations, "").whatever("unable to create dependency registrations")?;
            let workspace = temporary.path().join("workspace");
            for directory in ["inputs", "secrets"] {
                fs::create_dir_all(workspace.join(directory))
                    .whatever("unable to create rule workspace")?;
            }
            let mut mounts = vec![mount(&workspace, "/run/rugix/bakery/work", false)];
            mounts.extend_from_slice(project_mounts);
            mounts.extend([
                mount(
                    &registrations,
                    "/run/rugix/bakery/work/rebuild-if-changed.txt",
                    false,
                ),
                mount(&directory, "/run/rugix/bakery/work/rule", true),
                mount(&output_dir, "/run/rugix/bakery/work/outputs", false),
                mount(
                    &parameter_file,
                    "/run/rugix/bakery/work/parameters.json",
                    true,
                ),
            ]);
            for (name, path) in &inputs {
                mounts.push(mount(
                    path,
                    &format!("/run/rugix/bakery/work/inputs/{name}"),
                    true,
                ));
            }
            for (name, path) in &secrets {
                mounts.push(mount(
                    path,
                    &format!("/run/rugix/bakery/work/secrets/{name}"),
                    true,
                ));
            }
            let command: Vec<OsString> = task.run.command.iter().map(OsString::from).collect();
            let environment = [
                ("RUGIX_PROJECT_DIR", "/run/rugix/bakery/work/project"),
                (
                    "RUGIX_REBUILD_IF_CHANGED",
                    "/run/rugix/bakery/work/rebuild-if-changed.txt",
                ),
            ];
            let status = match &task.env {
                Some(env) => self.images[env].run_with_env(&command, &mounts, &environment)?,
                None => runtime::run(&command, &mounts, &environment)?,
            };
            if !status.success() {
                bail!("build rule {:?} failed", task.name);
            }
            let outputs = validate_outputs(task, &output_dir)?;
            let dependencies = ProjectDependencies::capture(&self.project, &registrations)?;
            if cached {
                let destination = self.cache.join(dependencies.cache_key(&key)?);
                if !destination.is_dir() {
                    fs::rename(&output_dir, &destination)
                        .whatever("unable to publish cached rule outputs")?;
                }
                let outputs = validate_outputs(task, &destination)?;
                dependencies.save(&dependency_record)?;
                outputs
            } else {
                self.temporary.push(temporary);
                outputs
            }
        };

        Ok(outputs)
    }
}

/// Reject missing outputs, incompatible kinds, and symlinks escaping /outputs.
fn validate_outputs(task: &RunTask, root: &Path) -> BakeryResult<BTreeMap<Str, PathBuf>> {
    let root = fs::canonicalize(root).whatever("unable to resolve rule outputs")?;
    let mut outputs = BTreeMap::new();
    for (name, output) in &task.run.outputs {
        let path = fs::canonicalize(root.join(&output.path))
            .whatever_with(|_| format!("rule {:?} did not produce output {name:?}", task.name))?;
        if !path.starts_with(&root) {
            bail!("rule output {name:?} escapes its output directory");
        }
        let metadata = fs::metadata(&path).whatever("unable to inspect rule output")?;
        let matches = match output.kind {
            ArtifactKind::File => metadata.is_file(),
            ArtifactKind::Directory => metadata.is_dir(),
        };
        if !matches {
            bail!("rule output {name:?} has an incompatible artifact kind");
        }
        outputs.insert(name.clone(), path);
    }
    Ok(outputs)
}

/// Length-prefix digest fields to avoid concatenation ambiguities.
fn push(key: &mut Sha256, name: &str, value: &str) {
    key.update((name.len() as u64).to_le_bytes());
    key.update(name.as_bytes());
    key.update((value.len() as u64).to_le_bytes());
    key.update(value.as_bytes());
}

/// Construct a bind mount without shell interpolation.
fn mount(source: &Path, target: &str, read_only: bool) -> BindMount {
    BindMount {
        source: source.to_owned(),
        target: target.into(),
        read_only,
    }
}
