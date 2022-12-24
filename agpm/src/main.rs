#![warn(missing_docs)]
//! This is the main execution file for my agpm project manager implementation.

use clap::Parser;
use color_eyre::eyre::{eyre, Result};

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
                agpm_pm::PMError::Git(_)| agpm_pm::PMError::Store(_)|agpm_pm::PMError::Interact(_) => {
                    eyre!(e).wrap_err(format!(
"Had a git or store error while installing. If the url is correct, run
`agpm clean`
and then install again"))
                },
                agpm_pm::PMError::FileExt(_) =>{ eyre!(e).wrap_err(format!(
"Error while moving files.
Check for read and write permissions in the directories:
    - {:?}
    - {:?}
Then manually move the files from the first directory to the second and run:
`agpm rebuild {{your project name}}`", PMDirsImpl::new().unwrap().git(), PMDirsImpl::new().unwrap().src()))
                },
                agpm_pm::PMError::Spawn(_)| agpm_pm::PMError::Exec => eyre!(e).wrap_err(
"Had some illegal arguments or problems with io, or failed at building.
Please edit with:
`agpm edit {{your project name}}`
And then run run:
`agpm rebuild {{your project name}}`"),
                _ => eyre!(e).wrap_err(
"Currently no fixes are available for your error. Try opening another terminal, then run:
`agpm clean`
and then try installing yout project again"),
        }})?,
        Commands::Uninstall { project } => {
            pm.i_uninstall(&[&project]).map_err(|e|{
            match e{
             agpm_pm::PMError::Spawn(_)| agpm_pm::PMError::Exec => eyre!(e).wrap_err(
"Had some illegal arguments or problems with io, or failed at building.
Please edit with:
`agpm edit {{project that failed}}`
And then run run:
`agpm uninstall {{all not uninstalled projects}}`"),
            _ => eyre!(e).wrap_err(
"Currently no fixes are available for your error. Try opening another terminal, then run:
`agpm clean`
and then try installing yout project again"),
            }
        })?},
        Commands::Update { project} => pm.i_update(&[& project])?,
        Commands::Restore { project } => pm.i_restore(&[&project])?,
        Commands::Reinstall { project } => pm.reinstall(&project)?,
        Commands::Rebuild { project } => pm.rebuild(&project)?,
        Commands::List { project } => pm.i_list(&[&project])?,
        Commands::Edit { project } => pm.i_edit(&project)?,
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
