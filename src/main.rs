use amisgitpm::{
    args::{Cli, Commands},
    build_suggestions::BuildSuggestions,
    interaction::{InstallInterImpl, MinorInterImpl, UpdateInterImpl},
    package_management::*,
};
use clap::Parser;

fn main() {
    let p = Cli::parse();
    match {match p.com {
        Commands::Install { url, path } => {
            PackageManager::inter_install::<BuildSuggestions, InstallInterImpl>(&url, path)
        }
        Commands::Uninstall { package } => PackageManager::uninstall(&package),
        Commands::Update { package } => PackageManager::inter_update::<UpdateInterImpl>(package),
        Commands::Restore { package } => PackageManager::restore(&package),
        Commands::Reinstall { package } => PackageManager::reinstall(&package),
        Commands::Rebuild { package } => PackageManager::rebuild(&package),
        Commands::List => PackageManager::list::<MinorInterImpl>(),
        Commands::Edit { package } => PackageManager::edit::<MinorInterImpl>(&package),
        Commands::Cleanup => PackageManager::cleanup(),
        Commands::Bootstrap => PackageManager::bootstrap(),
    }}{
        Err(e)=> println!("{e}"),
        _=>{},
    };
}

/*Suggestions */
