//! Rule execution, atomic result caching, and declared output validation.

use std::collections::{BTreeMap, HashMap};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use reportify::{bail, ResultExt};
use sha2::{Digest, Sha256};
use sidex::types::{Map, Str};
use tempfile::TempDir;

use crate::config::envs::EnvStorage;
use crate::config::rules::{ArtifactKind, RuleOutput};
use crate::envs::{BindMount, BuiltEnv, EnvStore};
use crate::project::envs::EnvDefinition;
use crate::BakeryResult;

use super::artifacts::Artifacts;
use super::cache::artifact_hash;
use super::layers::LayerTask;
use super::results::Results;
use super::secrets::SecretFingerprints;
use super::{runtime, ExecutionContext, Plan, RunTask, Task};

/// One build session. Uncached outputs remain alive until publication.
pub struct Executor {
    project: PathBuf,
    context: ExecutionContext,
    cache: Results,
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
        context: ExecutionContext,
    ) -> BakeryResult<Self> {
        let results = Results::new(cache, project)?;
        Ok(Self {
            project: fs::canonicalize(project).whatever("unable to resolve project directory")?,
            context,
            cache: results,
            env_cache: env_cache.to_owned(),
            images: HashMap::new(),
            storage,
            temporary: Vec::new(),
            artifacts: Artifacts::new(cache)?,
        })
    }

    /// Use the standard project-owned result cache and shared tooling environment store.
    pub fn for_project(
        project: &crate::project::ProjectRef,
        context: ExecutionContext,
    ) -> BakeryResult<Self> {
        let env_cache = Path::new("/run/rugix/bakery/cache")
            .join(project.local_id().as_str())
            .join("envs");
        Self::new(
            project.dir(),
            &project.dir().join(".rugix/rules"),
            &env_cache,
            EnvStorage::Auto,
            context,
        )
    }

    /// Preserve a standalone artifact whose path must outlive this session.
    pub fn keep_artifact(&mut self, artifact: &Path) {
        self.cache.keep_artifact(artifact);
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
            .open(self.cache.root().join("build.lock"))
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
                Task::Layer(task) => self.execute_layer(task, &results, bakery_identity)?,
                Task::Release(task) => {
                    let inputs = self.artifacts.resolve_all(
                        &BTreeMap::from([("archive".into(), task.archive.clone())]),
                        &results,
                    )?;
                    let temporary = tempfile::tempdir_in(self.cache.root())
                        .whatever("unable to create release workspace")?;
                    let outputs =
                        task.execute(&inputs["archive"], temporary.path(), self.context)?;
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
        key.update(b"rugix-artifact-build-v5\0");
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
            let fingerprints = SecretFingerprints::open(self.cache.root())?;
            for (name, path) in &secrets {
                push(
                    &mut key,
                    &format!("secret:{name}"),
                    &fingerprints.fingerprint(path)?,
                );
            }
        }
        let source_date_epoch = task
            .run
            .uses_source_date_epoch
            .unwrap_or(true)
            .then(|| self.context.source_date_epoch.to_string());
        if let Some(epoch) = &source_date_epoch {
            push(&mut key, "source-date-epoch", epoch);
        }
        let key = hex::encode(key.finalize());
        let images = &self.images;
        self.cache.execute(
            &format!("rule {:?}", task.name),
            cached.then_some(key.as_str()),
            &task.run.outputs,
            |workspace| {
                let output_dir = &workspace.outputs;
                let parameter_file = workspace.path().join("parameters.json");
                fs::write(
                    &parameter_file,
                    serde_json::to_vec(&task.parameters)
                        .whatever("unable to encode rule parameters")?,
                )
                .whatever("unable to write rule parameters")?;
                let registrations = &workspace.registrations;
                let workspace = workspace.path().join("workspace");
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
                    ("RUGIX_PROJECT_DIR", Some("/run/rugix/bakery/work/project")),
                    (
                        "RUGIX_REBUILD_IF_CHANGED",
                        Some("/run/rugix/bakery/work/rebuild-if-changed.txt"),
                    ),
                    ("SOURCE_DATE_EPOCH", source_date_epoch.as_deref()),
                ];
                let status = match &task.env {
                    Some(env) => images[env].run_with_env(&command, &mounts, &environment)?,
                    None => runtime::run(&command, &mounts, &environment)?,
                };
                if !status.success() {
                    bail!("build rule {:?} failed", task.name);
                }
                Ok(())
            },
        )
    }

    /// Fingerprint a layer's resolved definition and execute it through the shared cache.
    fn execute_layer(
        &mut self,
        task: &LayerTask,
        results: &[BTreeMap<Str, PathBuf>],
        runtime: Option<&str>,
    ) -> BakeryResult<BTreeMap<Str, PathBuf>> {
        let inputs = self.artifacts.resolve_all(&task.inputs, results)?;
        let mut key = Sha256::new();
        key.update(b"rugix-layer-build-v3\0");
        push(&mut key, "definition", &task.identity);
        push(
            &mut key,
            "runtime",
            runtime.unwrap_or("unidentified-bakery"),
        );
        let epoch = self.context.source_date_epoch;
        push(&mut key, "source-date-epoch", &epoch.to_string());
        for (name, path) in &inputs {
            push(&mut key, name, &self.artifacts.hash(path)?);
        }
        for (index, job) in task.jobs.iter().enumerate() {
            push(
                &mut key,
                &format!("recipe-path:{index}"),
                &job.recipe.path.to_string_lossy(),
            );
            let directory = fs::canonicalize(&job.recipe.path)
                .whatever("unable to resolve recipe implementation")?;
            push(
                &mut key,
                &format!("recipe:{index}"),
                &artifact_hash(&directory)?,
            );
            let parameters: BTreeMap<_, _> = job.parameters.iter().collect();
            push(
                &mut key,
                &format!("parameters:{index}"),
                &serde_json::to_string(&parameters)
                    .whatever("unable to encode recipe parameters")?,
            );
        }
        let key = hex::encode(key.finalize());
        let outputs = Map::from_iter([(
            "archive".into(),
            RuleOutput::new(ArtifactKind::File, "archive.tar"),
        )]);
        self.cache.execute(
            &format!("layer {:?}", task.name),
            runtime.map(|_| key.as_str()),
            &outputs,
            |workspace| task.execute(&inputs, &workspace.outputs, &workspace.registrations, epoch),
        )
    }
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
