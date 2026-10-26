//! Build and execute immutable tooling environments through Buildah.
//!
//! [`EnvStore::build`] resolves a Dockerfile definition to a [`BuiltEnv`] image.
//! Callers can run commands in fresh working containers without depending on the CLI.

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io::{self, IsTerminal};
use std::os::fd::AsFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use nix::errno::Errno;
use nix::sys::signal::{kill, killpg, Signal};
use nix::unistd::Pid;
use reportify::{bail, ResultExt};
use serde::Deserialize;
use tempfile::TempDir;
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::{info, warn};

use crate::config::envs::{EnvBuildInfo, EnvStorage};
use crate::project::envs::EnvDefinition;
use crate::BakeryResult;

/// Project-scoped image storage with temporary process-local runtime state.
#[derive(Clone, Debug)]
pub struct EnvStore {
    root: PathBuf,
    runtime: Arc<TempDir>,
    storage: EnvStorage,
}

impl EnvStore {
    /// Open a store. Auto selects native overlay when supported, otherwise VFS.
    #[tracing::instrument(skip_all, level = "debug")]
    pub fn new(cache: &Path, storage: EnvStorage) -> BakeryResult<Self> {
        fs::create_dir_all(cache).whatever("unable to create env cache directory")?;
        let cache = fs::canonicalize(cache).whatever("unable to resolve env cache directory")?;
        let storage = match storage {
            EnvStorage::Auto | EnvStorage::Native => {
                // Probe on the same filesystem without letting automatic FUSE detection
                // initialize the persistent native store with a different storage format.
                let probe_dir =
                    tempfile::tempdir_in(&cache).whatever("unable to create overlay probe")?;
                let probe = Self::open(probe_dir.path(), EnvStorage::Native)?;
                let output = execute(probe.command().arg("info").stdout(Stdio::piped()))?;
                let native = if output.status.success() {
                    let info: BuildahInfo = serde_json::from_slice(&output.stdout)
                        .whatever("unable to read Buildah storage information")?;
                    info.store
                        .graph_status
                        .get("Native Overlay Diff")
                        .is_some_and(|value| value == "true")
                } else {
                    false
                };
                if native {
                    EnvStorage::Native
                } else if storage == EnvStorage::Native {
                    bail!("native overlay is unavailable for the env cache; use --storage vfs or --storage fuse");
                } else {
                    info!("Native overlay is unavailable; using VFS for env storage.");
                    EnvStorage::Vfs
                }
            }
            storage => storage,
        };
        if storage == EnvStorage::Fuse && !Path::new("/dev/fuse").exists() {
            bail!("fuse-overlayfs requires /dev/fuse; pass the device into Bakery or use --storage vfs");
        }
        let directory = match storage {
            EnvStorage::Native => "native",
            EnvStorage::Fuse => "fuse",
            EnvStorage::Vfs => "vfs",
            EnvStorage::Auto => unreachable!("auto storage has been resolved"),
        };
        Self::open(&cache.join(directory), storage)
    }

    /// Build an environment and return the exact image selected by Buildah.
    #[tracing::instrument(skip_all)]
    pub fn build(&self, definition: &EnvDefinition, no_cache: bool) -> BakeryResult<BuiltEnv> {
        let build_dir = tempfile::tempdir_in(self.runtime.path())
            .whatever("unable to create env build directory")?;
        let image_file = build_dir.path().join("image.id");
        let mut command = self.command();
        command.args([
            "build",
            "--isolation=chroot",
            "--userns=host",
            "--network=host",
            "--pull=missing",
            "--layers",
            "--force-rm",
        ]);
        if no_cache {
            command.arg("--no-cache");
        }
        // Build output goes to stderr so stdout can identify the resolved image.
        command.stdout(Stdio::from(
            io::stderr()
                .as_fd()
                .try_clone_to_owned()
                .whatever("unable to duplicate stderr")?,
        ));
        command
            .arg("--iidfile")
            .arg(&image_file)
            .arg("--file")
            .arg(definition.context.join("Dockerfile"))
            .arg(&definition.context);
        require_success(execute(&mut command)?.status, "building env")?;
        let image_id =
            fs::read_to_string(image_file).whatever("unable to read built env image ID")?;
        let image_id = image_id.trim().to_owned();
        if image_id.is_empty() {
            bail!("Buildah returned an empty env image ID");
        }
        Ok(BuiltEnv {
            image_id,
            store: self.clone(),
        })
    }

    /// Open one explicitly selected backend.
    fn open(directory: &Path, storage: EnvStorage) -> BakeryResult<Self> {
        let root = directory.join("storage");
        fs::create_dir_all(&root).whatever("unable to create env image store")?;
        let runtime = Arc::new(
            tempfile::Builder::new()
                .prefix("rugix-envs-")
                .tempdir()
                .whatever("unable to create env runtime directory")?,
        );
        Ok(Self {
            root,
            runtime,
            storage,
        })
    }

    /// Prepare a Buildah invocation using only this store.
    fn command(&self) -> Command {
        let mut command = Command::new("buildah");
        command
            .arg("--root")
            .arg(&self.root)
            .arg("--runroot")
            .arg(self.runtime.path());
        match self.storage {
            EnvStorage::Native => {
                command.args([
                    "--storage-driver=overlay",
                    "--storage-opt=overlay.mount_program=",
                ]);
            }
            EnvStorage::Fuse => {
                command.args([
                    "--storage-driver=overlay",
                    "--storage-opt=overlay.mount_program=/usr/bin/fuse-overlayfs",
                ]);
            }
            EnvStorage::Vfs => {
                command.arg("--storage-driver=vfs");
            }
            EnvStorage::Auto => unreachable!("the store always has a resolved backend"),
        }
        command
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        command
    }
}

/// A resolved image together with its owning store.
#[derive(Debug)]
pub struct BuiltEnv {
    image_id: String,
    store: EnvStore,
}

impl BuiltEnv {
    /// Identity for build results and downstream cache keys.
    pub fn info(&self) -> EnvBuildInfo {
        EnvBuildInfo {
            image_id: self.image_id.clone().into(),
            storage: self.store.storage,
        }
    }

    /// Execute in a fresh container, preserving image defaults and cleaning up afterward.
    #[tracing::instrument(skip_all)]
    pub fn run(&self, arguments: &[OsString], mounts: &[BindMount]) -> BakeryResult<ExitStatus> {
        // Own the name before acquisition so cancellation during 'from' also cleans up.
        let container = WorkingContainer {
            id: format!("rugix-env-{}", uuid::Uuid::new_v4()),
            store: self.store.clone(),
        };
        let output = execute(
            self.store
                .command()
                .args(["from", "--isolation=chroot", "--userns=host", "--name"])
                .arg(&container.id)
                .arg(&self.image_id)
                .stdout(Stdio::null()),
        )?;
        require_success(output.status, "creating env working container")?;
        let mut command = self.store.command();
        command.args(["run", "--isolation=chroot"]);
        let terminal = io::stdin().is_terminal() && io::stdout().is_terminal();
        command.arg(if terminal {
            "--terminal=true"
        } else {
            "--terminal=false"
        });
        for mount in mounts {
            if mount.source.as_os_str().as_bytes().contains(&b':')
                || mount.target.as_os_str().as_bytes().contains(&b':')
            {
                bail!("env bind mount paths must not contain ':'");
            }
            let mut value = mount.source.as_os_str().to_os_string();
            value.push(":");
            value.push(&mount.target);
            value.push(if mount.read_only { ":ro" } else { ":rw" });
            command.arg("--volume").arg(value);
        }
        command
            .arg(&container.id)
            .arg("--")
            .args(arguments)
            .stdin(Stdio::inherit());
        let status = execute_with_terminal(&mut command, terminal)?.status;
        // Removal must finish before a caller publishes outputs or exits with the command's
        // status.
        container.remove()?;
        Ok(status)
    }
}

/// A directory or file exposed to a command through a bind mount.
#[derive(Debug)]
pub struct BindMount {
    pub source: PathBuf,
    pub target: PathBuf,
    pub read_only: bool,
}

/// Remove working containers during both normal completion and unwinding.
struct WorkingContainer {
    id: String,
    store: EnvStore,
}

impl WorkingContainer {
    /// Remove the container, reporting cleanup errors on normal completion.
    fn remove(mut self) -> BakeryResult<()> {
        let output = self
            .remove_command()
            .output()
            .whatever("unable to remove env working container")?;
        require_success(output.status, "removing env working container")?;
        self.id.clear();
        Ok(())
    }

    /// Cleanup also runs while a cancelled task unwinds, so it must not check
    /// cancellation.
    fn remove_command(&self) -> Command {
        let mut command = self.store.command();
        command
            .arg("rm")
            .arg(&self.id)
            .stdout(Stdio::null())
            .stderr(Stdio::piped());
        command
    }
}

impl Drop for WorkingContainer {
    fn drop(&mut self) {
        if self.id.is_empty() {
            return;
        }
        match self.remove_command().output() {
            Ok(output) if output.status.success() => {}
            Ok(output) => warn!(
                "unable to remove env working container: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
            Err(error) => warn!(%error, "unable to remove env working container"),
        }
    }
}

/// Only the upstream fields needed to qualify native storage are deserialized.
#[derive(Deserialize)]
struct BuildahInfo {
    store: BuildahStoreInfo,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BuildahStoreInfo {
    graph_status: HashMap<String, String>,
}

/// Run a cancelable subprocess without buffering build logs or terminal output.
fn execute(command: &mut Command) -> BakeryResult<Output> {
    execute_with_terminal(command, false)
}

/// Interactive Buildah must stay in the foreground group to access the controlling
/// terminal.
fn execute_with_terminal(command: &mut Command, terminal: bool) -> BakeryResult<Output> {
    rugix_tasks::check_canceled();
    if !terminal {
        command.process_group(0);
    }
    let mut command =
        tokio::process::Command::from(std::mem::replace(command, Command::new("buildah")));
    command.kill_on_drop(true);
    let mut child = command
        .spawn()
        .whatever("unable to start Buildah; ensure it is installed in Bakery")?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let mut process = BuildahProcess {
        child,
        group: !terminal,
        completed: false,
    };
    // Borrow the child while waiting so cancellation leaves it with the cleanup guard.
    let (status, stdout, stderr) = rugix_tasks::block_on(async {
        tokio::try_join!(process.child.wait(), capture(stdout), capture(stderr))
    })
    .whatever("unable to wait for Buildah")?;
    process.completed = true;
    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

/// Read only explicitly piped output. Build logs and terminal output are inherited.
async fn capture(pipe: Option<impl AsyncRead + Unpin>) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    if let Some(mut pipe) = pipe {
        pipe.read_to_end(&mut bytes).await?;
    }
    Ok(bytes)
}

/// Give Buildah time to remove intermediate containers before forcing termination.
struct BuildahProcess {
    child: tokio::process::Child,
    group: bool,
    completed: bool,
}

impl BuildahProcess {
    fn signal(&self, signal: Signal) {
        let Some(pid) = self.child.id().map(|pid| Pid::from_raw(pid as i32)) else {
            return;
        };
        let result = if self.group {
            killpg(pid, signal)
        } else {
            kill(pid, signal)
        };
        if let Err(error) = result {
            if error != Errno::ESRCH {
                warn!(%error, "unable to signal Buildah");
            }
        }
    }
}

impl Drop for BuildahProcess {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        self.signal(Signal::SIGTERM);
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            match self.child.try_wait() {
                Ok(Some(_)) => return,
                Ok(None) => std::thread::sleep(Duration::from_millis(20)),
                Err(error) => {
                    warn!(%error, "unable to wait for Buildah during cleanup");
                    break;
                }
            }
        }
        warn!("Buildah did not exit during cleanup; forcing termination");
        self.signal(Signal::SIGKILL);
    }
}

/// Retain the subprocess status in the error without logging its command arguments.
fn require_success(status: ExitStatus, operation: &str) -> BakeryResult<()> {
    if !status.success() {
        bail!("{operation} failed: {status}");
    }
    Ok(())
}
