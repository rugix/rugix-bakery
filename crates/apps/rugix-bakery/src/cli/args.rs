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
    /// Build an image, layer, or update bundle.
    #[clap(subcommand)]
    Bake(BakeCommand),
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

/// The `bake` command.
#[derive(Debug, Parser)]
pub enum InternalCommand {
    MakeImage {
        config: PathBuf,
        source: PathBuf,
        image: PathBuf,
    },
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
