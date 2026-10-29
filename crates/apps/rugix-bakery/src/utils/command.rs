//! Cancelable tooling subprocesses with terminal support and process-group cleanup.

use std::io;
use std::os::unix::process::CommandExt;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

use nix::errno::Errno;
use nix::sys::signal::{kill, killpg, Signal};
use nix::unistd::Pid;
use reportify::ResultExt;
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::warn;

use crate::BakeryResult;

/// Run a cancelable subprocess without buffering build logs or terminal output.
pub fn execute(command: &mut Command) -> BakeryResult<Output> {
    execute_with_terminal(command, false)
}

/// Interactive commands must stay in the foreground group to access the controlling
/// terminal.
pub fn execute_with_terminal(command: &mut Command, terminal: bool) -> BakeryResult<Output> {
    rugix_tasks::check_canceled();
    if !terminal {
        command.process_group(0);
    }
    let mut command =
        tokio::process::Command::from(std::mem::replace(command, Command::new("true")));
    command.kill_on_drop(true);
    let mut child = command
        .spawn()
        .whatever("unable to start tooling command")?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let mut process = CommandProcess {
        child,
        group: !terminal,
        completed: false,
    };
    // Borrow the child while waiting so cancellation leaves it with the cleanup guard.
    let (status, stdout, stderr) = rugix_tasks::block_on(async {
        tokio::try_join!(process.child.wait(), capture(stdout), capture(stderr))
    })
    .whatever("unable to wait for tooling command")?;
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

/// Allow a subprocess to clean up before forcing termination.
struct CommandProcess {
    child: tokio::process::Child,
    group: bool,
    completed: bool,
}

impl CommandProcess {
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
                warn!(%error, "unable to signal tooling command");
            }
        }
    }
}

impl Drop for CommandProcess {
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
                    warn!(%error, "unable to wait for tooling command during cleanup");
                    break;
                }
            }
        }
        warn!("tooling command did not exit during cleanup; forcing termination");
        self.signal(Signal::SIGKILL);
    }
}
