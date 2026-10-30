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
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Arc;

use reportify::{bail, ResultExt};
use serde::Deserialize;
use tempfile::TempDir;
use tracing::{info, warn};

use crate::config::envs::{EnvBuildInfo, EnvStorage};
use crate::project::envs::EnvDefinition;
use crate::utils::command::{execute, execute_with_terminal};
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
        self.configure_command(Command::new("buildah"))
    }

    /// Contain every process started by a chroot-isolated environment command.
    fn run_command(&self) -> Command {
        let mut command = Command::new("rugix-isolate");
        command.args(["--supervise", "--", "buildah"]);
        self.configure_command(command)
    }

    /// Apply the same storage backend and inherited process settings to both commands.
    fn configure_command(&self, mut command: Command) -> Command {
        // The node epoch must not alter environment image construction or Buildah itself.
        command.env_remove("SOURCE_DATE_EPOCH");
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
        self.run_with_env(arguments, mounts, &[])
    }

    /// Set or remove selected image variables for one command.
    #[tracing::instrument(skip_all)]
    pub fn run_with_env(
        &self,
        arguments: &[OsString],
        mounts: &[BindMount],
        environment: &[(&str, Option<&str>)],
    ) -> BakeryResult<ExitStatus> {
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
        if environment.iter().any(|(_, value)| value.is_none()) {
            let mut config = self.store.command();
            config.arg("config");
            for (name, value) in environment {
                if value.is_none() {
                    config.arg("--env").arg(format!("{name}-"));
                }
            }
            require_success(
                execute(config.arg(&container.id))?.status,
                "configuring env working container",
            )?;
        }
        // Storage helpers must outlive command supervision until container removal.
        require_success(
            execute(
                self.store
                    .command()
                    .arg("mount")
                    .arg(&container.id)
                    .stdout(Stdio::null()),
            )?
            .status,
            "mounting env working container",
        )?;
        let mut command = self.store.run_command();
        command.args(["run", "--isolation=chroot"]);
        for (name, value) in environment {
            if let Some(value) = value {
                command.arg("--env").arg(format!("{name}={value}"));
            }
        }
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
            value.push(":");
            value.push(mount.options()?);
            command.arg("--volume").arg(value);
        }
        command
            .arg(&container.id)
            .arg("--")
            .args(arguments)
            .stdin(Stdio::inherit());
        let status = execute_with_terminal(&mut command, terminal)?.status;
        // Removal must finish before a caller publishes outputs or exits with the
        // command's status.
        container.remove()?;
        Ok(status)
    }
}

/// A directory or file exposed to a command through a bind mount.
#[derive(Debug, Clone)]
pub struct BindMount {
    pub source: PathBuf,
    pub target: PathBuf,
    pub read_only: bool,
}

impl BindMount {
    /// Preserve inherited restrictions when remounting a bind in a rootless namespace.
    pub fn options(&self) -> BakeryResult<String> {
        let flags = nix::sys::statvfs::statvfs(&self.source)
            .whatever("unable to inspect bind mount restrictions")?
            .flags();
        let mut options = if self.read_only { "ro" } else { "rw" }.to_owned();
        for (flag, option) in [
            (nix::sys::statvfs::FsFlags::ST_NOSUID, ",nosuid"),
            (nix::sys::statvfs::FsFlags::ST_NODEV, ",nodev"),
            (nix::sys::statvfs::FsFlags::ST_NOEXEC, ",noexec"),
        ] {
            if flags.contains(flag) {
                options.push_str(option);
            }
        }
        Ok(options)
    }
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

/// Retain the subprocess status in the error without logging its command arguments.
fn require_success(status: ExitStatus, operation: &str) -> BakeryResult<()> {
    if !status.success() {
        bail!("{operation} failed: {status}");
    }
    Ok(())
}
