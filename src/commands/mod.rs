pub mod new;

use crate::{commands::new::NewCommand, types::CommandResult, ui::UserInterface};
use clap::Subcommand;

pub trait Command {
    /// Run/executes the given command
    fn run(self, ui: &dyn UserInterface) -> CommandResult<()>;
}

/// All possible commands in the Lit CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    New(NewCommand),
}

impl Commands {
    /// Gets command name
    pub fn get_name(&self) -> &'static str {
        return match self {
            Commands::New(_) => "New",
        };
    }
}

impl Command for Commands {
    fn run(self, ui: &dyn UserInterface) -> CommandResult<()> {
        return match self {
            Commands::New(cmd) => cmd.run(ui),
        };
    }
}
