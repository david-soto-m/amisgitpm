use amisgitpm::{
    args::{Cli, Commands},
    build_suggestions::BuildSuggestions,
    interaction::{MinorInteractionsImpl, UserInstallInteractions},
    package_management::{PackageManagement, PackageManager},
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let p = Cli::parse();
    match dbg!(p).com {
        Commands::Install { url, .. } => {
            PackageManager::interactive_install::<BuildSuggestions, UserInstallInteractions>(&url)
        }
        Commands::Uninstall { package } => PackageManager::uninstall(&package),
        Commands::List {} => PackageManager::list::<MinorInteractionsImpl>(),
        Commands::Edit { package } => PackageManager::edit::<MinorInteractionsImpl>(&package),
        Commands::Cleanup {  } => PackageManager::cleanup(),
        _ => todo!(),
    }
    .unwrap();
}

/*Suggestions */
