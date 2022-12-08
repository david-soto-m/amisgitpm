#![warn(missing_docs)]
//! This is the main execution file for my agpm package manager implementation.

use clap::Parser;
use color_eyre::eyre::{eyre, Result, WrapErr};

use agpm::{
    args::{Cli, Commands},
    prelude::*,
    PMDirsImpl, Project, ProjectManager, UpdatePolicy,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();
    let mut pm = ProjectManager::new().unwrap();
    match args.com {
        Commands::Install { url } => pm.i_install(&url).map_err(|e|{
            match e{
                agpm_pm::PMError::Git(_) => {
                    eyre!(e).wrap_err(
"Had a git error while installing.
Try running `agpm clean` and then installing again")
                },
                agpm_pm::PMError::FileExt(_) =>{ eyre!(e).wrap_err(format!(
"Error while moving files.
Check for read and write permissions in the directories:
    - {:?}
    - {:?}", PMDirsImpl::new().unwrap().git(), PMDirsImpl::new().unwrap().src()))
                },
                agpm_pm::PMError::Spawn(_) => eyre!(e).wrap_err(""),
                _ => eyre!(e).wrap_err(
"You had a really weird error, please open an issue at https://github.com/david-soto-m/amisgitpm/issues
with the whole error message"),
        }})?,
        Commands::Uninstall { package } => pm.i_uninstall(&[&package])?,
        Commands::Update { package} => pm.i_update(&[& package])?,
        Commands::Restore { package } => pm.i_restore(&[&package])?,
        Commands::Reinstall { package } => pm.reinstall(&package)?,
        Commands::Rebuild { package } => pm.rebuild(&package)?,
        Commands::List { package } => pm.i_list(&[&package])?,
        Commands::Edit { package } => pm.i_edit(&package)?,
        Commands::Clean => pm.cleanup()?,
        Commands::Bootstrap => {
            let prj = Project {
                name: "amisgitpm".into(),
                dir: "amisgitpm".into(),
                url: "https://github.com/david-soto-m/amisgitpm.git".into(),
                ref_string: "refs/heads/main".into(),
                update_policy: UpdatePolicy::Always,
                install_script: vec!["cargo install --path . --root ~/.local/".into()],
                uninstall_script: vec!["cargo uninstall amisgitpm --root ~/.local/".into()],
            };
            pm.install(prj)?
        }

    };
    Ok(())
}
