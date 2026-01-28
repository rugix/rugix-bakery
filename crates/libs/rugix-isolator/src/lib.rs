//! Process isolation using Linux namespaces.
//!
//! This library provides an [`Isolator`] struct for forking a child process into an
//! isolated set of Linux namespaces. Upon isolation, the execution will continue in the
//! child process. The parent will wait for the child, forward signals, and exit with the
//! child's exit status when the child exits. This essentially transfers the runtime
//! context and ongoing execution to the child.
//!
//! ## Safety Considerations
//!
//! Forking a process in a multi-threaded program is inherently problematic, as it may
//! lead to deadlocks and inconsistent state. For this reason, the isolator checks that
//! the process is single-threaded before forking, and returns an error if it is not.
//! Furthermore, as the parent does not return to the caller on success, any destructors
//! or cleanup code in the parent will not be executed, which should prevent any sort of
//! interference with the child's execution enabling a clean and safe transfer of control.
//!
//! ## Isolation Details
//!
//! The isolator creates a new user and mount namespace for the child process. Optionally,
//! a new PID namespace can be created. The parent process writes the appropriate
//! `uid_map` and `gid_map` to allow the child to appear as root (UID 0) inside the new
//! user namespace and also enables subordinate UID/GID ranges for container-like use of
//! users/groups.

use std::ffi::CString;
use std::os::unix::io::{AsRawFd, OwnedFd};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI32, Ordering};

/// Isolator for forking a process into an isolated environment.
pub struct Isolator {
    bind_mounts: Vec<BindMount>,
    chroot_path: Option<PathBuf>,
    new_pid_namespace: bool,
}

impl Isolator {
    /// Create a new isolator with default settings.
    pub fn new() -> Self {
        Self {
            bind_mounts: Vec::new(),
            chroot_path: None,
            new_pid_namespace: false,
        }
    }

    /// Add a bind mount to set up in the isolated child.
    ///
    /// The mount is created after the mount namespace is set up but before chroot (if
    /// configured).
    pub fn with_bind_mount(mut self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Self {
        self.bind_mounts.push(BindMount {
            src: src.as_ref().to_path_buf(),
            dst: dst.as_ref().to_path_buf(),
            recursive: false,
        });
        self
    }

    /// Add a recursive bind mount to set up in the isolated child.
    ///
    /// The mount is created after the mount namespace is set up but before chroot (if
    /// configured).
    pub fn with_recursive_bind_mount(
        mut self,
        src: impl AsRef<Path>,
        dst: impl AsRef<Path>,
    ) -> Self {
        self.bind_mounts.push(BindMount {
            src: src.as_ref().to_path_buf(),
            dst: dst.as_ref().to_path_buf(),
            recursive: true,
        });
        self
    }

    /// Set a chroot path for the isolated child.
    ///
    /// After bind mounts are set up, the child will chroot to this path.
    pub fn with_chroot(mut self, path: impl AsRef<Path>) -> Self {
        self.chroot_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Spawn the child in a new PID namespace.
    pub fn with_new_pid_namespace(mut self) -> Self {
        self.new_pid_namespace = true;
        self
    }

    /// Transfer the execution into an isolated child process.
    ///
    /// **On success, this function only returns in the child process.** The parent waits
    /// and then exits with the child's status code.
    pub fn isolate(&self) -> Result<(), IsolateError> {
        // For safety, we need to ensure that we are single-threaded before forking and
        // transferring control. Otherwise, we run the risk of deadlocks, inconsistencies,
        // and all sorts of other issues that may arise with multi-threaded forking.
        match is_single_threaded() {
            Some(true) => {}
            Some(false) => {
                return Err(IsolateError::new("process is multi-threaded"));
            }
            None => {
                return Err(IsolateError::new("unable to determine thread count"));
            }
        }

        let parent_uid_map = std::fs::read_to_string("/proc/self/uid_map")
            .map_err(|e| IsolateError::new("unable to read parent 'uid_map'").with_source(e))?;
        let parent_gid_map = std::fs::read_to_string("/proc/self/gid_map")
            .map_err(|e| IsolateError::new("unable to read parent 'gid_map'").with_source(e))?;

        // The child needs to wait for the parent to write the `uid_map`/`gid_map`, so we create
        // a pipe for synchronization. The parent will drop its write end after writing the maps,
        // and the child will read from the read end to wait for the parent.
        let (read_fd, write_fd) = nix::unistd::pipe()
            .map_err(|e| IsolateError::new("unable to create pipe").with_source(e))?;

        let mut clone_flags =
            nix::libc::CLONE_NEWUSER | nix::libc::CLONE_NEWNS | nix::libc::SIGCHLD;
        if self.new_pid_namespace {
            clone_flags |= nix::libc::CLONE_NEWPID;
        }

        // We may be writing to stdout/stderr from the parent process, so we need to flush
        // the buffers before forking in order to ensure no output is duplicated.
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();

        let pid = unsafe {
            // SAFETY: We have verified that we are single-threaded.
            nix::libc::syscall(nix::libc::SYS_clone, clone_flags, std::ptr::null::<()>())
        };

        if pid < 0 {
            let error = std::io::Error::last_os_error();
            return Err(IsolateError::new("unable to clone process").with_source(error));
        }

        if pid == 0 {
            drop(write_fd);
            self.child_setup(read_fd)?;
            Ok(())
        } else {
            drop(read_fd);
            self.parent_run(pid as u32, write_fd, &parent_uid_map, &parent_gid_map)
        }
    }

    /// Run the parent process logic: write uid/gid maps, forward signals, wait for child.
    fn parent_run(
        &self,
        child_pid: u32,
        write_fd: OwnedFd,
        parent_uid_map: &str,
        parent_gid_map: &str,
    ) -> Result<(), IsolateError> {
        let pidfd = unsafe { nix::libc::syscall(nix::libc::SYS_pidfd_open, child_pid, 0i32) };
        if pidfd >= 0 {
            CHILD_PIDFD.store(pidfd as i32, Ordering::SeqCst);
        } else {
            eprintln!(
                "isolation failed: unable to open pidfd for child process: {}",
                std::io::Error::last_os_error()
            );
            // Not sure how to gracefully recover from this, so we just kill the process.
            std::process::exit(1);
        }

        if let Err(error) = self.write_maps(child_pid, parent_uid_map, parent_gid_map) {
            let result = unsafe { nix::libc::kill(child_pid as i32, nix::libc::SIGKILL) };
            if result != 0 {
                eprintln!(
                    "isolation failed: {error}, unable to kill child process: {}",
                    std::io::Error::last_os_error()
                );
                std::process::exit(1);
            }
            wait_for_child(child_pid);
            return Err(error);
        }

        unsafe {
            for &sig in FORWARDED_SIGNALS {
                nix::libc::signal(
                    sig,
                    forward_signal_handler as *const () as nix::libc::sighandler_t,
                );
            }
        }

        // Signal the child that setup is complete by closing the write end of the pipe.
        drop(write_fd);

        match wait_for_child(child_pid) {
            ChildExitReason::Exited(code) => std::process::exit(code),
            ChildExitReason::Signaled(signal) => std::process::exit(128 + signal),
        }
    }

    /// Write the `uid_map` and `gid_map` for the child process.
    fn write_maps(
        &self,
        child_pid: u32,
        parent_uid_map: &str,
        parent_gid_map: &str,
    ) -> Result<(), IsolateError> {
        // We transform the parent map into an identity map here to get the same mapping as the
        // parent. This is required to propagate subordinate UID/GID ranges correctly when the
        // parent is already in a user namespace.
        let uid_map = make_identity_map(parent_uid_map);
        let uid_map_path = format!("/proc/{child_pid}/uid_map");
        let uid_result = std::fs::write(&uid_map_path, &uid_map);

        if let Err(error) = &uid_result
            && error.raw_os_error() == Some(nix::libc::EPERM)
        {
            // Apparently, we cannot write the maps directly, so we fall back to using the
            // `newuidmap` and `newgidmap` helpers. This allows the isolator to be used in a
            // rootless context where the user has been granted subordinate UID/GID ranges.
            return self.write_maps_with_helpers(child_pid);
        }
        uid_result.map_err(|e| {
            IsolateError::new(format!("failed to write {uid_map_path}")).with_source(e)
        })?;

        // Same identity transformation for `gid_map`.
        let gid_map = make_identity_map(parent_gid_map);
        let gid_map_path = format!("/proc/{child_pid}/gid_map");
        std::fs::write(&gid_map_path, &gid_map).map_err(|e| {
            IsolateError::new(format!("failed to write {gid_map_path}")).with_source(e)
        })?;

        Ok(())
    }

    /// Write the `uid_map` and `gid_map` for the child process using the `newuidmap` and
    /// `newgidmap` helpers.
    fn write_maps_with_helpers(&self, child_pid: u32) -> Result<(), IsolateError> {
        let real_uid = nix::unistd::getuid().as_raw();
        let real_gid = nix::unistd::getgid().as_raw();

        let username = get_username(real_uid).ok_or_else(|| {
            IsolateError::new(format!("unable to get username for uid {real_uid}"))
        })?;

        let subuid = parse_subid_file("/etc/subuid", &username)
            .map_err(|e| IsolateError::new("failed to parse '/etc/subuid'").with_source(e))?
            .ok_or_else(|| {
                IsolateError::new(format!("no subuid entry found for uid {real_uid}"))
            })?;

        let subgid = parse_subid_file("/etc/subgid", &username)
            .map_err(|e| IsolateError::new("failed to parse '/etc/subgid'").with_source(e))?
            .ok_or_else(|| {
                IsolateError::new(format!("no subgid entry found for uid {real_uid}"))
            })?;

        let status = std::process::Command::new("newuidmap")
            .arg(child_pid.to_string())
            .arg("0")
            .arg(real_uid.to_string())
            .arg("1")
            .arg("1")
            .arg(subuid.start.to_string())
            .arg(subuid.count.to_string())
            .status()
            .map_err(|e| IsolateError::new("failed to execute 'newuidmap'").with_source(e))?;
        if !status.success() {
            return Err(IsolateError::new(format!(
                "'newuidmap' failed with exit code {}",
                status.code().unwrap_or(-1)
            )));
        }

        let status = std::process::Command::new("newgidmap")
            .arg(child_pid.to_string())
            .arg("0")
            .arg(real_gid.to_string())
            .arg("1")
            .arg("1")
            .arg(subgid.start.to_string())
            .arg(subgid.count.to_string())
            .status()
            .map_err(|e| IsolateError::new("failed to execute 'newgidmap'").with_source(e))?;
        if !status.success() {
            return Err(IsolateError::new(format!(
                "newgidmap failed with exit code {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    /// Child setup after clone: set up mount namespace, bind mounts, chroot.
    fn child_setup(&self, read_fd: OwnedFd) -> Result<(), IsolateError> {
        let mut buf = [0u8; 1];
        nix::unistd::read(read_fd.as_raw_fd(), &mut buf)
            .map_err(|e| IsolateError::new("unable to wait for parent").with_source(e))?;

        let result = unsafe {
            nix::libc::mount(
                std::ptr::null(),
                c"/".as_ptr(),
                std::ptr::null(),
                nix::libc::MS_REC | nix::libc::MS_PRIVATE,
                std::ptr::null(),
            )
        };
        if result < 0 {
            let error = std::io::Error::last_os_error();
            return Err(IsolateError::new("unable to make '/' private").with_source(error));
        }

        for bind_mount in &self.bind_mounts {
            self.setup_bind_mount(bind_mount)?;
        }
        if let Some(ref chroot_path) = self.chroot_path {
            self.setup_chroot(chroot_path)?;
        }
        Ok(())
    }

    /// Setup a bind mount in the child process.
    fn setup_bind_mount(&self, bind_mount: &BindMount) -> Result<(), IsolateError> {
        let src = CString::new(bind_mount.src.as_os_str().as_encoded_bytes()).map_err(|e| {
            IsolateError::new(format!(
                "invalid source path: '{}'",
                bind_mount.src.display()
            ))
            .with_source(e)
        })?;
        let dst = CString::new(bind_mount.dst.as_os_str().as_encoded_bytes()).map_err(|e| {
            IsolateError::new(format!(
                "invalid destination path: '{}'",
                bind_mount.dst.display()
            ))
            .with_source(e)
        })?;
        let mut flags = nix::libc::MS_BIND;
        if bind_mount.recursive {
            flags |= nix::libc::MS_REC;
        }
        let result = unsafe {
            nix::libc::mount(
                src.as_ptr(),
                dst.as_ptr(),
                std::ptr::null(),
                flags,
                std::ptr::null(),
            )
        };
        if result < 0 {
            let error = std::io::Error::last_os_error();
            return Err(IsolateError::new(format!(
                "bind mount '{}' -> '{}' failed",
                bind_mount.src.display(),
                bind_mount.dst.display(),
            ))
            .with_source(error));
        }
        Ok(())
    }

    /// Setup chroot in the child process.
    fn setup_chroot(&self, path: &Path) -> Result<(), IsolateError> {
        nix::unistd::chroot(path).map_err(|e| {
            IsolateError::new(format!("unable to chroot to '{}'", path.display())).with_source(e)
        })?;
        nix::unistd::chdir("/").map_err(|e| {
            IsolateError::new("unable to change directory to '/' after chroot").with_source(e)
        })?;
        Ok(())
    }
}

/// A subordinate ID range from `/etc/subuid` or `/etc/subgid`.
struct SubIdRange {
    start: u32,
    count: u32,
}

/// Parse `/etc/subuid` or `/etc/subgid` to find the subordinate ID range for a user.
///
/// The file format is: `name:start:count` or `id:start:count`
fn parse_subid_file(path: &str, username: &str) -> Result<Option<SubIdRange>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    for line in content.lines() {
        let mut parts = line.split(':');
        if parts.next().map(str::trim) != Some(username) {
            continue;
        }
        let Some(start) = parts.next().and_then(|s| s.trim().parse::<u32>().ok()) else {
            continue;
        };
        let Some(count) = parts.next().and_then(|s| s.trim().parse::<u32>().ok()) else {
            continue;
        };
        return Ok(Some(SubIdRange { start, count }));
    }
    Ok(None)
}

/// Get the username for a given UID.
fn get_username(uid: u32) -> Option<String> {
    let info = unsafe { nix::libc::getpwuid(uid) };
    if info.is_null() {
        return None;
    }
    unsafe { CString::from_raw((*info).pw_name) }
        .into_string()
        .ok()
}

/// Convert a `uid_map`/`gid_map` to an identity mapping.
fn make_identity_map(map: &str) -> String {
    let mut result = String::new();
    for line in map.lines() {
        let mut parts = line.split_whitespace();
        let Some(container_id) = parts.next() else {
            continue;
        };
        // Skip the ID in the parent namespace.
        parts.next();
        let Some(count) = parts.next() else {
            continue;
        };
        result.push_str(container_id);
        result.push(' ');
        result.push_str(container_id);
        result.push(' ');
        result.push_str(count);
        result.push('\n');
    }
    if result.is_empty() {
        "0 0 1\n".to_string()
    } else {
        result
    }
}

/// A bind mount to set up in the isolated child.
#[derive(Debug, Clone)]
struct BindMount {
    src: PathBuf,
    dst: PathBuf,
    recursive: bool,
}

/// Error transferring the execution to an isolated child process.
#[derive(Debug)]
pub struct IsolateError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl IsolateError {
    /// Create a new isolate error with the given message.
    fn new<M: std::fmt::Display>(message: M) -> Self {
        Self {
            message: message.to_string(),
            source: None,
        }
    }

    /// Set the source error for this isolate error.
    fn with_source<E>(mut self, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

impl std::fmt::Display for IsolateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "isolation failed: {}", self.message)
    }
}

impl std::error::Error for IsolateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|source| &**source as _)
    }
}

/// Check if the current process is single-threaded.
fn is_single_threaded() -> Option<bool> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(count_str) = line.strip_prefix("Threads:") {
            let count = count_str.trim().parse::<u32>().ok()?;
            return Some(count == 1);
        }
    }
    None
}

/// Child process exit reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChildExitReason {
    Exited(i32),
    Signaled(i32),
}

/// Wait for child `pid` to exit and forward its exit status.
fn wait_for_child(child_pid: u32) -> ChildExitReason {
    loop {
        let mut status: i32 = 0;
        let result = unsafe { nix::libc::waitpid(child_pid as i32, &mut status, 0) };
        if result < 0 {
            let wait_error = std::io::Error::last_os_error();
            eprintln!("isolation failed: unable to wait for child process: {wait_error}");
            std::process::exit(1);
        }
        if nix::libc::WIFEXITED(status) {
            return ChildExitReason::Exited(nix::libc::WEXITSTATUS(status));
        } else if nix::libc::WIFSIGNALED(status) {
            return ChildExitReason::Signaled(nix::libc::WTERMSIG(status));
        }
    }
}

/// Global child `pidfd` for signal forwarding.
///
/// Note that each parent has at most one child during its entire lifetime.
static CHILD_PIDFD: AtomicI32 = AtomicI32::new(-1);

/// Signal handler that forwards signals to the child process using `pidfd`.
extern "C" fn forward_signal_handler(sig: i32) {
    let pidfd = CHILD_PIDFD.load(Ordering::SeqCst);
    if pidfd >= 0 {
        unsafe {
            nix::libc::syscall(
                nix::libc::SYS_pidfd_send_signal,
                pidfd,
                sig,
                std::ptr::null::<nix::libc::siginfo_t>(),
                0i32,
            );
        }
    }
}

/// Signals to forward to the child process.
const FORWARDED_SIGNALS: &[i32] = &[
    nix::libc::SIGTERM,
    nix::libc::SIGINT,
    nix::libc::SIGHUP,
    nix::libc::SIGQUIT,
    nix::libc::SIGUSR1,
    nix::libc::SIGUSR2,
];
