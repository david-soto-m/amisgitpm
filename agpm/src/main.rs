#![warn(missing_docs)]
//!

use agpm_lib::{prelude::*, ProjectManager};
use clap::Parser;

pub mod args;
use crate::args::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    let mut pm = ProjectManager::new().unwrap();
    match args.com {
        Commands::Install { url } => pm.inter_install(&url),
        Commands::Uninstall { package } => pm.uninstall(&package),
        Commands::Update { package, force } => pm.inter_update(package, force),
        Commands::Restore { package } => pm.restore(&package),
        Commands::Reinstall { package } => pm.reinstall(&package),
        Commands::Rebuild { package } => pm.rebuild(&package),
        Commands::List { package } => pm.list(package),
        Commands::Edit { package } => pm.inter_edit(&package),
        Commands::Cleanup => pm.cleanup(),
        Commands::Bootstrap => pm.bootstrap(),
    }
    .unwrap_or_else(|e| println!("{e}"));
}
