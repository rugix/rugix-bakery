//! Definition of the command line arguments.

use std::path::PathBuf;

use clap::Parser;

use crate::config::systems::Architecture;
use crate::oven::system::ReleaseInfo;
use crate::oven::{BundleOpts, MixinSelection};

/// Command line arguments.
#[derive(Debug, Parser)]
#[command(author, version = rugix_version::RUGIX_GIT_VERSION, about = None, long_about = None)]
pub struct Args {
    /// Path to the `rugix-bakery.toml` configuration file.
    #[clap(long)]
    pub config: Option<PathBuf>,
    /// The command to execute.
    #[clap(subcommand)]
    pub cmd: Command,
}

/// Commands of the CLI.
#[derive(Debug, Parser)]
pub enum Command {
    /// Internal implementation entry points.
    #[clap(subcommand, hide = true)]
    Internal(InternalCommand),
    /// Build an image, layer, or update bundle.
    #[clap(subcommand)]
    Bake(BakeCommand),
    /// Inspect, build, and enter tooling environments.
    #[clap(subcommand)]
    Envs(EnvsCommand),
    /// Discover and inspect repository-defined build rules.
    #[clap(subcommand)]
    Rules(DefinitionsCommand),
    /// Discover and inspect repository recipes.
    #[clap(subcommand)]
    Recipes(DefinitionsCommand),
    /// Discover and inspect layers and their architecture variants.
    #[clap(subcommand)]
    Layers(DefinitionsCommand),
    /// Run system tests.
    Test(TestCommand),
    /// Run a system in a VM.
    Run(RunCommand),
    /// List systems, recipes, and layers.
    #[clap(subcommand)]
    List(ListCommand),
    /// Pull in external repositories.
    Pull,
    /// Initialize the project from a template.
    Init(InitCommand),
    /// Spawn a shell in the Rugix Bakery Docker container.
    Shell,
    /// Control the cache of Rugix Bakery.
    #[clap(subcommand)]
    Cache(CacheCommand),
    /// Run Rugix Bundler.
    Bundler(BundlerCommand),
}

#[derive(Debug, clap::Args)]
pub struct ReleaseInfoArgs {
    #[clap(long)]
    pub release_id: Option<String>,
    #[clap(long)]
    pub release_version: Option<String>,
}

impl ReleaseInfoArgs {
    pub fn release_info(&self) -> ReleaseInfo {
        ReleaseInfo {
            system_id: self.release_id.clone(),
            system_version: self.release_version.clone(),
        }
    }
}

/// The `list` command.
#[derive(Debug, Parser)]
pub enum ListCommand {
    /// List available images.
    Systems,
}

/// Inspect repository definitions without building them.
#[derive(Debug, Parser)]
pub enum DefinitionsCommand {
    /// List visible names, or export their definitions as a JSON catalog.
    List {
        /// Restrict to a repository alias, core, or . for the project itself.
        #[clap(long)]
        repository: Option<String>,
        #[clap(long)]
        json: bool,
    },
    /// Show a definition and its reference documentation.
    Inspect {
        name: String,
        #[clap(long)]
        json: bool,
    },
}

/// Commands for independent tooling environments.
#[derive(Debug, Parser)]
pub enum EnvsCommand {
    /// List envs in the project and its direct repositories.
    List,
    /// Show an env definition without building it.
    Inspect {
        env: String,
        /// Print the inspection result as JSON.
        #[clap(long)]
        json: bool,
    },
    /// Build an env and print its resolved image ID.
    Build(EnvBuildArgs),
    /// Build an env and start a fresh shell, or run a command after --.
    Shell {
        #[clap(flatten)]
        build: EnvBuildArgs,
        /// Command to execute instead of /bin/sh.
        #[clap(last = true)]
        command: Vec<std::ffi::OsString>,
    },
}

/// Arguments shared by env construction and shell entry.
#[derive(Debug, clap::Args)]
pub struct EnvBuildArgs {
    /// Local or repository-qualified env name.
    pub env: String,
    /// Image storage backend.
    #[clap(long, value_enum, default_value = "auto")]
    pub storage: crate::config::envs::EnvStorage,
    /// Rebuild Dockerfile instructions without using cached layers.
    #[clap(long)]
    pub no_cache: bool,
}

/// Parameter file arguments for overriding recipe parameters.
#[derive(Debug, clap::Args)]
pub struct ParamFileArgs {
    /// Parameter files to override recipe parameters.
    #[clap(long = "param-file")]
    pub param_files: Vec<PathBuf>,
}

/// Build-time mixin selection arguments.
#[derive(Debug, clap::Args)]
pub struct MixinArgs {
    /// Enable a mixin for this bake in addition to system defaults.
    #[clap(long = "enable-mixin")]
    pub enable_mixins: Vec<String>,
    /// Disable a default mixin for this bake.
    #[clap(long = "disable-mixin")]
    pub disable_mixins: Vec<String>,
    /// Do not enable mixins configured on the system by default.
    #[clap(long)]
    pub no_default_mixins: bool,
}

impl MixinArgs {
    pub fn selection(&self) -> MixinSelection {
        MixinSelection {
            enable: self.enable_mixins.clone(),
            disable: self.disable_mixins.clone(),
            no_default_mixins: self.no_default_mixins,
        }
    }
}

/// The `bake` command.
#[derive(Debug, Parser)]
pub enum BakeCommand {
    /// Execute a rule invocation from a TOML file.
    Rule {
        /// Invocation with uses, inputs, parameters, and optional secrets.
        invocation: PathBuf,
        /// Directory in which to publish named outputs.
        output: PathBuf,
    },
    /// Bake a system
    Image {
        /// The name of the system to bake.
        system: String,
        /// The output path for the resulting files.
        output: Option<PathBuf>,
        #[clap(flatten)]
        release: ReleaseInfoArgs,
        #[clap(long)]
        source_date: Option<jiff::Timestamp>,
        #[clap(flatten)]
        params: ParamFileArgs,
        #[clap(flatten)]
        mixins: MixinArgs,
    },
    /// Bake a layer.
    Layer {
        /// The architecture to bake the layer for.
        #[clap(long)]
        arch: Architecture,
        /// The name of the layer to bake.
        layer: String,
        #[clap(long)]
        source_date: Option<jiff::Timestamp>,
        #[clap(flatten)]
        params: ParamFileArgs,
    },
    /// Bake a bundle.
    Bundle {
        system: String,
        output: Option<PathBuf>,
        /// Disable compression of the bundle.
        #[clap(flatten)]
        opts: BundleOpts,
        #[clap(flatten)]
        release: ReleaseInfoArgs,
        #[clap(flatten)]
        params: ParamFileArgs,
        #[clap(flatten)]
        mixins: MixinArgs,
    },
}

/// The `test` command.
#[derive(Debug, Parser)]
pub struct TestCommand {
    pub workflows: Vec<String>,
    #[clap(flatten)]
    pub params: ParamFileArgs,
}

/// The `cache` command.
#[derive(Debug, Parser)]
pub enum CacheCommand {
    /// Clean the cache.
    Clean,
}

/// The `run` command.
#[derive(Debug, Parser)]
pub struct RunCommand {
    #[clap(flatten)]
    pub release: ReleaseInfoArgs,
    pub system: String,
    #[clap(flatten)]
    pub params: ParamFileArgs,
    #[clap(flatten)]
    pub mixins: MixinArgs,
}

/// Implementation entry points used by repository rules.
#[derive(Debug, Parser)]
pub enum InternalCommand {
    /// Execute a core rule using Bakery's standard rule mounts.
    ExecRule { name: String },
}

/// The `init` command.
#[derive(Debug, Parser)]
pub struct InitCommand {
    /// Template to use.
    pub template: Option<String>,
}

/// The `bundler` command.
#[derive(Debug, Parser)]
pub struct BundlerCommand {
    #[clap(allow_hyphen_values(true))]
    pub args: Vec<String>,
}
