//! This is the library associated with the amisgitpm.
//!
//! The idea of this library is to make programmatic interactions with the
//! project as painless as possible.
//! To make everything easy to mix and match there is a preference for
//! trait based interfaces.

use agpm_lib::{ProjectManager, prelude::*};
use clap::Parser;

pub mod args;
use crate::args::{Cli, Commands};

pub fn matcher(args: Cli, pm: &mut (impl PMBasics + PMExtended + PMInteractive)) {
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

fn main() {
    let args = Cli::parse();
    let mut pm = ProjectManager::new().unwrap();
    matcher(args, &mut pm);
}
