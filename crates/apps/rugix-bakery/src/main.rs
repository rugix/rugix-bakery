//! Rugix Bakery executable.

use clap::Parser;

use reportify::Report;

pub mod build;
pub mod cli;
pub mod config;
pub mod envs;
pub mod oven;
pub mod paths;
pub mod project;
pub mod rules;
pub mod tester;
pub mod utils;

reportify::new_whatever_type! {
    /// Error running Rugix Bakery.
    pub BakeryError
}

/// [`Result`] with [`Report<BakeryError>`] as error type.
pub type BakeryResult<T> = Result<T, Report<BakeryError>>;

/// Entrypoint of the executable.
pub fn main() {
    if let Err(error) = rugix_isolator::Isolator::new().isolate() {
        eprintln!("warning: unable to isolate process: {error}");
        eprintln!("continuing without isolation...");
    } else {
        utils::emulation::mark_isolated();
    }
    rugix_cli::CliBuilder::new().run(|| cli::run(cli::args::Args::parse()))
}
