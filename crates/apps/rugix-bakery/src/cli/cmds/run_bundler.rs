//! The `bundler` command.

use std::ffi::CString;
use std::ops::Deref;
use std::os::unix::ffi::OsStrExt;

use reportify::ResultExt;

use crate::cli::args::BundlerCommand;
use crate::{paths, BakeryResult};

/// Run the `bundler` command.
pub fn run(cmd: &BundlerCommand) -> BakeryResult<()> {
    let bundler_path = paths::bundler_path();
    let mut args = vec![CString::new(bundler_path.as_os_str().as_bytes()).unwrap()];
    for arg in &cmd.args {
        args.push(CString::new(arg.as_bytes()).unwrap());
    }
    let args = args.iter().map(|arg| arg.deref()).collect::<Vec<_>>();
    // Replace ourselves with Rugix Bundler.
    nix::unistd::execvp::<&std::ffi::CStr>(&args[0], &args)
        .whatever("error executing Rugix Bundler")?;
    Ok(())
}
