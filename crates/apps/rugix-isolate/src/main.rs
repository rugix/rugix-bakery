//! Spawn a process in isolated environment.

use std::ffi::CString;
use std::path::PathBuf;

use clap::Parser;
use rugix_isolator::Isolator;

#[derive(Debug, Parser)]
#[clap(about = "Spawn a process in isolated environment.")]
pub struct Args {
    /// Bind mount a source path to a destination path (format: src:dst).
    #[clap(long = "bind", value_name = "SRC:DST")]
    bind_mounts: Vec<String>,

    /// Recursive bind mount a source path to a destination path (format: src:dst).
    #[clap(long = "rbind", value_name = "SRC:DST")]
    recursive_bind_mounts: Vec<String>,

    /// Chroot to the specified path.
    #[clap(long)]
    chroot: Option<PathBuf>,

    /// Create a new PID namespace.
    #[clap(long)]
    pid_namespace: bool,

    /// Command to execute.
    #[clap(required = true, trailing_var_arg = true)]
    command: Vec<String>,
}

/// Entrypoint of the executable.
fn main() {
    let args = Args::parse();

    if let Err(error) = run(args) {
        eprintln!("rugix-isolate: {error}");
        std::process::exit(1);
    }
}

/// Create an isolated child process and spawn the specified command in it.
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut isolator = Isolator::new();

    for bind in &args.bind_mounts {
        let (src, dst) = parse_bind_mount(bind)?;
        isolator = isolator.with_bind_mount(src, dst);
    }
    for bind in &args.recursive_bind_mounts {
        let (src, dst) = parse_bind_mount(bind)?;
        isolator = isolator.with_recursive_bind_mount(src, dst);
    }
    if let Some(ref chroot_path) = args.chroot {
        isolator = isolator.with_chroot(chroot_path);
    }
    if args.pid_namespace {
        isolator = isolator.with_new_pid_namespace();
    }

    isolator.isolate()?;

    exec_command(&args.command)?;

    Ok(())
}

/// Parse a bind mount specification in the format "src:dst".
fn parse_bind_mount(spec: &str) -> Result<(PathBuf, PathBuf), String> {
    let parts: Vec<&str> = spec.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "invalid bind mount specification '{spec}': expected format 'src:dst'"
        ));
    }
    Ok((PathBuf::from(parts[0]), PathBuf::from(parts[1])))
}

/// Execute the specified command, replacing the current process.
fn exec_command(command: &[String]) -> Result<(), String> {
    if command.is_empty() {
        return Err("no command specified".to_string());
    }

    let prog = CString::new(command[0].as_str()).map_err(|e| format!("invalid program: {e}"))?;
    let args: Vec<CString> = command
        .iter()
        .map(|s| CString::new(s.as_str()).unwrap())
        .collect();
    let args: Vec<*const libc::c_char> = args
        .iter()
        .map(|s| s.as_ptr())
        .chain(std::iter::once(std::ptr::null()))
        .collect();

    unsafe {
        libc::execvp(prog.as_ptr(), args.as_ptr());
    }

    Err(format!("exec failed: {}", std::io::Error::last_os_error()))
}
