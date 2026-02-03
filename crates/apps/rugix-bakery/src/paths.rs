//! Runtime/configurable paths for host-side tooling and shared assets.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::config::systems::Architecture;

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).map(PathBuf::from)
}

fn env_or_default(name: &str, default: &str, compile_time: Option<&'static str>) -> PathBuf {
    env_path(name)
        .or_else(|| compile_time.map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from(default))
}

static SHARE_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Base directory that contains Rugix shared assets (boot files, templates,
/// repositories).
pub fn share_dir() -> &'static Path {
    SHARE_DIR
        .get_or_init(|| {
            env_or_default(
                "RUGIX_BAKERY_SHARE_DIR",
                "/usr/share/rugix",
                option_env!("RUGIX_BAKERY_SHARE_DIR"),
            )
        })
        .as_path()
}

/// Directory with project templates.
pub fn templates_dir() -> PathBuf {
    share_dir().join("templates")
}

/// Directory with bundled repositories.
pub fn repositories_dir() -> PathBuf {
    share_dir().join("repositories")
}

/// Directory with boot assets.
pub fn boot_dir() -> PathBuf {
    share_dir().join("boot")
}

/// Directory with Raspberry Pi firmware.
pub fn pi_firmware_dir() -> PathBuf {
    share_dir().join("pi").join("firmware")
}

static BUNDLER: OnceLock<PathBuf> = OnceLock::new();

/// Path (or name) of the Rugix Bundler executable.
pub fn bundler_path() -> &'static Path {
    BUNDLER
        .get_or_init(|| {
            env_or_default(
                "RUGIX_BUNDLER",
                "rugix-bundler",
                option_env!("RUGIX_BUNDLER"),
            )
        })
        .as_path()
}

static SHELL: OnceLock<PathBuf> = OnceLock::new();

/// Path (or name) of the shell to spawn for `rugix-bakery shell`.
pub fn shell_path() -> &'static Path {
    SHELL
        .get_or_init(|| {
            env_path("RUGIX_SHELL")
                .or_else(|| env_path("SHELL"))
                .unwrap_or_else(|| PathBuf::from("/bin/bash"))
        })
        .as_path()
}

static OVMF_AMD64: OnceLock<PathBuf> = OnceLock::new();
static OVMF_ARM64: OnceLock<PathBuf> = OnceLock::new();

/// Path to the UEFI firmware used by QEMU tests.
pub fn ovmf_code_path(arch: Architecture) -> &'static Path {
    match arch {
        Architecture::Amd64 => OVMF_AMD64
            .get_or_init(|| {
                env_or_default(
                    "RUGIX_OVMF_CODE_AMD64",
                    "/usr/share/OVMF/OVMF_CODE.fd",
                    option_env!("RUGIX_OVMF_CODE_AMD64"),
                )
            })
            .as_path(),
        Architecture::Arm64 => OVMF_ARM64
            .get_or_init(|| {
                env_or_default(
                    "RUGIX_OVMF_CODE_ARM64",
                    "/usr/share/AAVMF/AAVMF_CODE.fd",
                    option_env!("RUGIX_OVMF_CODE_ARM64"),
                )
            })
            .as_path(),
        _ => Path::new(""),
    }
}
