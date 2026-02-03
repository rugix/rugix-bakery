//! Rugix Bakery executable.

use clap::Parser;

use reportify::Report;

pub mod cli;
pub mod config;
pub mod oven;
pub mod paths;
pub mod project;
pub mod tester;
pub mod utils;

reportify::new_whatever_type! {
    /// Error running Rugix Bakery.
    BakeryError
}

/// [`Result`] with [`Report<BakeryError>`] as error type.
pub type BakeryResult<T> = Result<T, Report<BakeryError>>;

/// Entrypoint of the executable.
pub fn main() {
    if let Err(error) = rugix_isolator::Isolator::new().isolate() {
        eprintln!("warning: unable to isolate process: {error}");
        eprintln!("continuing without isolation...");
    }
    rugix_cli::CliBuilder::new().run(|| cli::run(cli::args::Args::parse()))
}
