use crate::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
/// Lit CLI - A tool to scaffold and manage Lit projects
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
