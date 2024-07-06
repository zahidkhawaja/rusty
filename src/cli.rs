use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a bash command
    Query { query: String },
    /// Setup the configuration file
    Setup,
    /// Show the history of queries
    History,
}
