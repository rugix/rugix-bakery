use std::path::Path;

use reportify::{bail, ResultExt};

use rugix_common::boot::grub::grub_write_defaults;

use crate::config::systems::{Architecture, SystemConfig};
use crate::{paths, BakeryResult};

pub fn initialize_grub<'cx>(config: &SystemConfig, config_dir: &Path) -> BakeryResult<()> {
    rugix_fs::create_dir_recursive(&config_dir.join("EFI/BOOT")).ok();
    rugix_fs::create_dir_recursive(&config_dir.join("rugpi")).ok();
    let mut copier = rugix_fs::Copier::new();
    copier
        .copy_file(
            &paths::boot_dir().join("grub/cfg/first.grub.cfg"),
            &config_dir.join("rugpi/grub.cfg"),
        )
        .whatever("unable to copy first stage boot script")?;
    grub_write_defaults(config_dir).whatever("unable to write Grub default environment")?;
    match config.architecture {
        Architecture::Arm64 => {
            copier
                .copy_file(
                    &paths::boot_dir().join("grub/bin/BOOTAA64.efi"),
                    &config_dir.join("EFI/BOOT/BOOTAA64.efi"),
                )
                .whatever("unable to copy Grub binary")?;
        }
        Architecture::Amd64 => {
            copier
                .copy_file(
                    &paths::boot_dir().join("grub/bin/BOOTX64.efi"),
                    &config_dir.join("EFI/BOOT/BOOTX64.efi"),
                )
                .whatever("unable to copy Grub binary")?;
        }
        _ => {
            bail!(
                "no Grub support for architecture `{}`",
                config.architecture.as_str()
            );
        }
    }
    Ok(())
}
