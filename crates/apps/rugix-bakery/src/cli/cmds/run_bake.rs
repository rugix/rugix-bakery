//! The `bake` command.

use std::path::Path;

use reportify::ResultExt;

use crate::cli::{args, load_project};
use crate::oven::{self, LayerBakery};
use crate::BakeryResult;

/// Run the `bake` command.
pub fn run(args: &args::Args, cmd: &args::BakeCommand) -> BakeryResult<()> {
    let project = load_project(args)?;
    match cmd {
        args::BakeCommand::Rule { invocation, output } => {
            crate::build::build_invocation(&project, invocation, output)?;
        }
        args::BakeCommand::Image {
            system,
            output,
            release,
            source_date,
            params,
            mixins,
        } => {
            let param_overrides = oven::load_parameter_overrides(&params.param_files)?;
            let system_path = Path::new("build").join(system);
            let source_date_epoch = source_date.map(|date| date.as_second() as u64);
            oven::bake_system(
                &project,
                &release.release_info(),
                system,
                &system_path,
                source_date_epoch,
                &param_overrides,
                &mixins.selection(),
                None,
            )?;
            if let Some(output) = output {
                crate::build::copy_file(&system_path.join("system.img"), output)?;
            }
        }
        args::BakeCommand::Layer {
            layer,
            arch,
            source_date,
            params,
        } => {
            let param_overrides = oven::load_parameter_overrides(&params.param_files)?;
            let source_date_epoch = source_date.map(|date| date.as_second() as u64);
            LayerBakery::new(&project, *arch, &param_overrides)
                .bake_root(layer, source_date_epoch)?;
        }
        args::BakeCommand::Bundle {
            system,
            output,
            opts,
            release,
            params,
            mixins,
        } => {
            let param_overrides = oven::load_parameter_overrides(&params.param_files)?;
            let system_path = Path::new("build").join(system);
            oven::bake_system(
                &project,
                &release.release_info(),
                system,
                &system_path,
                None,
                &param_overrides,
                &mixins.selection(),
                Some(opts),
            )?;
            let output = output
                .clone()
                .unwrap_or_else(|| system_path.join("system.rugixb"));
            let hash = oven::bake_bundle(&project, system, &system_path, &output, opts)?;
            println!("{hash}");
            let hash_output = output.with_extension("rugixb-hash");
            std::fs::write(&hash_output, hash.to_string())
                .whatever("unable to write bundle hash")?;
        }
    }
    Ok(())
}
