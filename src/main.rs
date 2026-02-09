mod cli;
mod logging;

use clap::Parser;

use crate::cli::Cli;
use crate::logging::set_verbose;

fn main() {
    let cli = Cli::parse();

    set_verbose(cli.verbose);

    vlog!("Hello, world!");
}
