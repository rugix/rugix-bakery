//! Run a command to completion and reap its entire process tree without changing PIDs.

use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;
use std::{fs, io};

use nix::errno::Errno;
use nix::sys::signal::{SaFlags, SigAction, SigHandler, SigSet, Signal, kill, sigaction};
use nix::sys::wait::waitpid;
use nix::unistd::Pid;

use crate::IsolateError;

static STOP_SIGNAL: AtomicI32 = AtomicI32::new(0);

/// Run one command and terminate all descendants before returning its status.
///
/// The caller must be single-threaded and have no existing child processes.
/// PID and procfs views remain unchanged. Signal handlers are installed for this
/// supervisor process, which should exit after this function returns.
pub fn supervise(command: &[String]) -> Result<ExitStatus, IsolateError> {
    if crate::is_single_threaded() != Some(true) {
        return Err(IsolateError::new("supervisor must be single-threaded"));
    }
    supervise_inner(command)
        .map_err(|error| IsolateError::new("unable to supervise command").with_source(error))
}

/// Record termination requests without interrupting descendant cleanup.
extern "C" fn stop(signal: i32) {
    STOP_SIGNAL.store(signal, Ordering::Relaxed);
}

fn supervise_inner(command: &[String]) -> io::Result<ExitStatus> {
    let (program, arguments) = command
        .split_first()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no command specified"))?;
    nix::sys::prctl::set_child_subreaper(true)?;
    STOP_SIGNAL.store(0, Ordering::Relaxed);
    // Unreaped children retain their PIDs until every PID in a snapshot is signaled.
    unsafe {
        sigaction(
            Signal::SIGCHLD,
            &SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty()),
        )?;
        let action = SigAction::new(
            SigHandler::Handler(stop),
            SaFlags::SA_RESTART,
            SigSet::empty(),
        );
        for signal in [Signal::SIGTERM, Signal::SIGINT, Signal::SIGHUP] {
            sigaction(signal, &action)?;
        }
    }
    let mut child = Command::new(program).args(arguments).spawn()?;
    let result = loop {
        let signal = STOP_SIGNAL.load(Ordering::Relaxed);
        if signal != 0 {
            break Ok(ExitStatus::from_raw(signal));
        }
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => break Err(error),
        }
    };
    terminate_descendants()?;
    result
}

/// Killing one generation reparents its survivors to this subreaper.
fn terminate_descendants() -> io::Result<()> {
    let children = format!("/proc/self/task/{}/children", std::process::id());
    loop {
        let snapshot = fs::read_to_string(&children)?;
        for pid in snapshot.split_whitespace() {
            let pid = pid
                .parse::<i32>()
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
            match kill(Pid::from_raw(pid), Signal::SIGKILL) {
                Ok(()) | Err(Errno::ESRCH) => {}
                Err(error) => return Err(error.into()),
            }
        }
        match waitpid(Pid::from_raw(-1), None) {
            Ok(_) | Err(Errno::EINTR) => {}
            Err(Errno::ECHILD) => return Ok(()),
            Err(error) => return Err(error.into()),
        }
    }
}
