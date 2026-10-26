//! Thin CLI access to environment discovery, construction, and fresh execution.

use std::ffi::OsString;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};

use reportify::ResultExt;

use crate::cli::{args, load_project};
use crate::envs::{BindMount, EnvStore};
use crate::project::envs::Envs;
use crate::BakeryResult;

/// Run an environment command without involving recipes, layers, or systems.
pub fn run(args: &args::Args, command: &args::EnvsCommand) -> BakeryResult<()> {
    let project = load_project(args)?;
    let repositories = project.repositories()?;
    let envs = Envs::load(repositories.clone())?;
    match command {
        args::EnvsCommand::List => rugix_cli::suspend(|| {
            for name in envs.available().keys() {
                println!("{name}");
            }
        }),
        args::EnvsCommand::Inspect { env, json } => {
            let info = envs.lookup(repositories.root_repository, env)?.info(env);
            let output = if *json {
                serde_json::to_string_pretty(&info)
                    .whatever("unable to serialize env information")?
            } else {
                format!(
                    "Env: {}\nDockerfile: {}\nContext: {}",
                    info.name, info.dockerfile, info.context
                )
            };
            rugix_cli::suspend(|| println!("{output}"));
        }
        args::EnvsCommand::Build(build) | args::EnvsCommand::Shell { build, .. } => {
            let definition = envs.lookup(repositories.root_repository, &build.env)?;
            rugix_cli::hide_status();
            let cache = Path::new("/run/rugix/bakery/cache")
                .join(project.local_id().as_str())
                .join("envs");
            let store = EnvStore::new(&cache, build.storage)?;
            let image = store.build(definition, build.no_cache)?;
            match command {
                args::EnvsCommand::Build(_) => println!("{}", image.info().image_id),
                args::EnvsCommand::Shell { command, .. } => {
                    let arguments = if command.is_empty() {
                        vec![OsString::from("/bin/sh")]
                    } else {
                        command.clone()
                    };
                    let status = image.run(
                        &arguments,
                        &[BindMount {
                            source: project.dir().to_path_buf(),
                            target: PathBuf::from("/project"),
                            read_only: false,
                        }],
                    )?;
                    // Release temporary runtime state before exiting with the command's status.
                    drop(image);
                    drop(store);
                    std::process::exit(
                        status
                            .code()
                            .unwrap_or_else(|| 128 + status.signal().unwrap_or(1)),
                    );
                }
                _ => unreachable!("only build and shell reach environment construction"),
            }
        }
    }
    Ok(())
}
