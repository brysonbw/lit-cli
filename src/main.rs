//! > **A command line interface to scaffold and manage Lit projects.**
//!
//! ## Install
//!
//! ```bash
//! cargo install lit-cli
//! ```
//!
//! ## Usage
//!
//! ```bash
//! lit-cli <COMMAND>
//! ```
//!
//! ### Commands
//!
//! | Command | Description                                               |
//! | ------- | --------------------------------------------------------- |
//! | new     | Create a new project                                      |
//! | help    | Print this message or the help of the given subcommand(s) |
//!
//! ### Options
//!
//! | Option        | Description   |
//! | ------------- | ------------- |
//! | -h, --help    | Print help    |
//! | -V, --version | Print version |

mod cli;
mod commands;
mod models;
mod types;
mod ui;

use std::process;

use chroma_print::{print_error, print_info};
use clap::{Error, Parser, error::ErrorKind};

use crate::{cli::Cli, commands::Command, ui::terminal::TerminalUserInterface};

fn main() {
    let cli = Cli::try_parse().unwrap_or_else(|error: Error| match error.kind() {
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
            print_info!("{}", error);
            process::exit(0);
        }
        _ => {
            print_error!("Error: {}", error);
            process::exit(1);
        }
    });

    let ui = TerminalUserInterface::new();
    let cmd_name: &str = cli.command.get_name();

    if let Err(error) = cli.command.run(&ui) {
        print_error!("'{}' Command Error: {}", cmd_name, error);
        process::exit(1);
    }
}
