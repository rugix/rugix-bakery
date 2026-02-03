//! The `shell` command.

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;

use reportify::ResultExt;

use crate::{paths, BakeryResult};

/// Run the `shell` command.
pub fn run() -> BakeryResult<()> {
    // Replace ourselves with a shell. This is primarily intended for debugging.
    let shell_path = paths::shell_path();
    let shell = CString::new(shell_path.as_os_str().as_bytes()).unwrap();
    nix::unistd::execvp::<&std::ffi::CStr>(&shell, &[shell.as_c_str()])
        .whatever("error executing shell")?;
    Ok(())
}
