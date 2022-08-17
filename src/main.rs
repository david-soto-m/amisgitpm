use amisgitpm::{
    args::{Cli, Commands},
    build_suggestions::BuildSuggestions,
    interaction::{InstallInteractionsImpl, MinorInteractionsImpl},
    package_management::{PackageManagement, PackageManager},
};
use clap::Parser;

fn main() {
    let p = Cli::parse();
    match p.com {
        Commands::Install { url, path } => {
            PackageManager::interactive_install::<BuildSuggestions, InstallInteractionsImpl>(&url, path )
        }
        Commands::Uninstall { package } => PackageManager::uninstall(&package),
        Commands::Reinstall { package } => PackageManager::reinstall(&package),
        Commands::Rebuild { package } => PackageManager::rebuild(&package),
        Commands::List => PackageManager::list::<MinorInteractionsImpl>(),
        Commands::Edit { package } => PackageManager::edit::<MinorInteractionsImpl>(&package),
        Commands::Cleanup => PackageManager::cleanup(),
        Commands::Bootstrap => PackageManager::bootstrap(),
        _ => todo!(),
    }
    .unwrap();
}

/*Suggestions */
