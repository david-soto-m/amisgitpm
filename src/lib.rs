// #![warn(missing_docs)]

//! This is the library associated with the amisgitpm.
//!
//! The idea of this library is to make programmatic interactions with the
//! project as painless as possible.
//! To make everything easy to mix and match there is a preference for
//! trait based interfaces.

pub mod args;
pub mod build_suggestions;
pub mod dirutils;
pub mod interaction;
pub mod package_management;
pub mod projects;
use crate::{
    args::{Cli, Commands},
    build_suggestions::BuildSuggester,
    interaction::{MinorInteractions, InstallInteractions},
    package_management::{
        PackageManagementCore, PackageManagementExt, PackageManagementInteractive,
    },
};

pub fn matcher<T, Q, R>(args: Cli, pm: T, inter: Q)
where
T: PackageManagementCore + PackageManagementExt + PackageManagementInteractive,
Q: InstallInteractions + MinorInteractions,
R: BuildSuggester,
{
    match args.com {
        Commands::Install { url } => {
            pm.inter_install::<_, R>(&url, inter)
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
