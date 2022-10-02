use amisgitpm::{
    args::{Cli, Commands},
    build_suggestions::BuildSuggestions,
    interaction::Interactor,
    package_management::{
        PackageManagementCore, PackageManagementExt, PackageManagementInteractive, PackageManager,
    },
};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let pm = PackageManager {};
    let inter = Interactor{};
    match args.com {
        Commands::Install { url } => {
            pm.inter_install::<Interactor, BuildSuggestions>(&url, inter)
                .unwrap_or_else(|e| println!("{e}"));
        }
        Commands::Uninstall { package } => {
            pm.uninstall(&package).unwrap_or_else(|e| println!("{e}"))
        }
        Commands::Update { package, force } => pm
            .inter_update(package, force, inter)
            .unwrap_or_else(|e| println!("{e}")),
        Commands::Restore { package } => pm.restore(&package).unwrap_or_else(|e| println!("{e}")),
        Commands::Reinstall { package } => {
            pm.reinstall(&package).unwrap_or_else(|e| println!("{e}"))
        }
        Commands::Rebuild { package } => pm.rebuild(&package).unwrap_or_else(|e| println!("{e}")),
        Commands::Rename { old_name, new_name } => {
            pm.rename(&old_name, &new_name)
                .unwrap_or_else(|e| println!("{e}"));
        }
        Commands::List { package } => {
            pm.list(package, inter)
                .unwrap_or_else(|e| println!("{e}"));
        }
        Commands::Edit { package } => {
            pm.edit(&package, inter)
                .unwrap_or_else(|e| println!("{e}"));
        }
        Commands::Cleanup => {
            pm.cleanup().unwrap_or_else(|e| println!("{e}"));
        }
        Commands::Bootstrap => {
            pm.bootstrap().unwrap_or_else(|e| println!("{e}"));
        }
    }
}

/*Suggestions */
