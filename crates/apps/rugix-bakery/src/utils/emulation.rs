//! Foreign-architecture execution in Bakery's private user namespace.
//!
//! Linux 6.7 and newer allow a private binfmt_misc registry. Registering the
//! distribution's fixed-interpreter QEMU configuration here enables existing
//! chroot-based recipes without changing the host's interpreter registry.

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use nix::mount::{mount, MsFlags};
use reportify::{bail, ResultExt};

use crate::config::systems::Architecture;
use crate::BakeryResult;

static ISOLATED: AtomicBool = AtomicBool::new(false);

/// Called only after startup has created a private user and mount namespace.
pub fn mark_isolated() {
    ISOLATED.store(true, Ordering::Relaxed);
}

/// Enable an emulator before both cached and uncached foreign layer builds.
pub fn prepare(architecture: Architecture) -> BakeryResult<()> {
    let emulator = match (std::env::consts::ARCH, architecture) {
        ("x86_64", Architecture::Amd64) | ("aarch64", Architecture::Arm64) => return Ok(()),
        (_, Architecture::Amd64) => "qemu-x86_64",
        (_, Architecture::Arm64) => "qemu-aarch64",
        (_, Architecture::Armv7 | Architecture::Armhf | Architecture::Arm) => "qemu-arm",
    };
    let host_registry = Path::new("/proc/sys/fs/binfmt_misc");
    if fs::read_to_string(host_registry.join("status"))
        .is_ok_and(|status| status.trim() == "enabled")
        && fs::read_to_string(host_registry.join(emulator)).is_ok_and(|entry| {
            entry.lines().next() == Some("enabled")
                && entry.lines().any(|line| {
                    line.strip_prefix("flags: ")
                        .is_some_and(|flags| flags.contains('F'))
                })
        })
    {
        return Ok(());
    }
    if !ISOLATED.load(Ordering::Relaxed) {
        bail!("foreign-architecture builds require successful namespace isolation or a preconfigured QEMU interpreter");
    }
    let registry = Path::new("/run/rugix/bakery/binfmt");
    if !registry.join("status").is_file() {
        fs::create_dir_all(registry).whatever("unable to create private interpreter registry")?;
        mount(Some("binfmt_misc"), registry, Some("binfmt_misc"), MsFlags::empty(), None::<&str>)
            .whatever("unable to mount private interpreter registry; use a Linux 6.7+ builder, configure host emulation, or build on the target architecture")?;
    }
    if registry.join(emulator).is_file() {
        return Ok(());
    }
    let configuration = fs::read_to_string(format!("/usr/lib/binfmt.d/{emulator}.conf")).whatever(
        "unable to read QEMU interpreter configuration; install qemu-user-static in Bakery",
    )?;
    let entry = configuration
        .lines()
        .find(|line| line.starts_with(&format!(":{emulator}:")))
        .ok_or_else(|| reportify::whatever!("missing QEMU interpreter registration"))?;
    if !entry
        .rsplit(':')
        .next()
        .is_some_and(|flags| flags.contains('F'))
    {
        bail!("QEMU interpreter registration must retain the interpreter across chroot");
    }
    fs::write(registry.join("register"), entry)
        .whatever("unable to register private QEMU interpreter")?;
    Ok(())
}
