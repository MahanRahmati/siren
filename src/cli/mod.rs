use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "siren")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = concat!("Siren v", env!("CARGO_PKG_VERSION")))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Use verbose output
    #[arg(short, long, default_value_t = false, global = true)]
    pub verbose: bool,

    /// Output result in JSON format
    #[arg(short = 'j', long, default_value_t = false)]
    pub output_json: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Reset configuration to default values
    ResetConfig,
}
