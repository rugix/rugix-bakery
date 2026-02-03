use std::path::Path;

use reportify::ResultExt;

use rugix_common::fsutils::copy_recursive;

use crate::{paths, BakeryResult};

pub fn initialize_tryboot(config_dir: &Path) -> BakeryResult<()> {
    copy_recursive(paths::boot_dir().join("tryboot"), &config_dir)
        .whatever("unable to initialize tryboot")?;
    Ok(())
}
