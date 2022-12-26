#![warn(missing_docs)]
//! This is the main execution file for my agpm project manager implementation.

use clap::Parser;
use color_eyre::eyre::{eyre, Result};

use agpm::{
    args::{Cli, Commands},
    prelude::*,
    PMDirsImpl, Project, ProjectManager, UpdatePolicy,
};
use agpm_pm::PMError;
use amisgitpm::CommonPMErrors;

const NO_FIX: &str = "Currently no fixes are available for your error";

const NON_EXIST: (&str, &str) = (
    "There are no projects installed with name",
    "Use `agpm list` to see all available projects",
);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();
    let mut pm = ProjectManager::new().unwrap();
    match args.com {
        Commands::Install { url } => pm.i_install(&url).map_err(|e| match e {
            PMError::Git(_) | PMError::Store(_) => eyre!(e).wrap_err(format!(
                "Had a git or store error while installing. If the url is correct, run
`agpm clean`
and then install again"
            )),
            PMError::FileExt(_) => eyre!(e).wrap_err(format!(
                "Error while moving files.
Check for read and write permissions in the directories:
    - {:?}
    - {:?}
Then manually move the files from the first directory to the second and run:
`agpm rebuild {{your project name}}`",
                PMDirsImpl::new().unwrap().git(),
                PMDirsImpl::new().unwrap().src()
            )),
            PMError::Spawn(_) | PMError::Exec => eyre!(e).wrap_err(
                "Had some illegal arguments or problems with io, or failed at building.
Please edit with:
`agpm edit {{your project name}}`
And then run run:
`agpm rebuild {{your project name}}`",
            ),
            _ => eyre!(e).wrap_err(NO_FIX),
        })?,
        Commands::Uninstall { project } => pm.i_uninstall(&project).map_err(|e| match e {
            PMError::Spawn(_) | PMError::Exec => eyre!(e).wrap_err(
                "Had some illegal arguments or problems with io, or failed at building.
Please edit with:
`agpm edit {{project that failed}}`
And then run run:
`agpm uninstall {{all not uninstalled projects}}`",
            ),
            PMError::Common(CommonPMErrors::NonExisting) => {
                eyre!(e).wrap_err(format!("{} {:?}\n{}", NON_EXIST.0, project, NON_EXIST.1))
            }
            PMError::IO(_) => eyre!(e).wrap_err(format!(
                "Error while erasing files, check the permissions for the directories:
    - {:?}
    - {:?}
and run again. If you run into this same problem and you know that the files are
erased, then go to:
    - {:?}
and manually delete the file that has the name of the program that's giving you trouble
",
                PMDirsImpl::new().unwrap().src(),
                PMDirsImpl::new().unwrap().old(),
                PMDirsImpl::new().unwrap().projects_db()
            )),
            PMError::Store(_) => eyre!(e).wrap_err(format!(
                "Had a store error go to:
    - {:?}
and manually delete the file that has the name of the program that's giving you trouble",
                PMDirsImpl::new().unwrap().projects_db()
            )),
            _ => eyre!(e).wrap_err(NO_FIX),
        })?,
        Commands::Update { project } => pm.i_update(&project).map_err(|e| match e {
            PMError::Git(e) => eyre!(e).wrap_err(format!(
                "Error while updating with git.
Solve the git problems manually in the corresponding directory in:
    - {:?}
Then manually move it to:
    - {:?}
and run
`agpm rebuild {{your project name}}`",
                PMDirsImpl::new().unwrap().git(),
                PMDirsImpl::new().unwrap().src(),
            )),
            PMError::Common(CommonPMErrors::NonExisting) => {
                eyre!(e).wrap_err(format!("{} {:?}\n{}", NON_EXIST.0, project, NON_EXIST.1))
            }
            PMError::FileExt(_) => eyre!(e).wrap_err(format!(
                "Error while move files, check the permissions for the directories:
    - {:?}
    - {:?}
    - {:?}
and run again.
",
                PMDirsImpl::new().unwrap().git(),
                PMDirsImpl::new().unwrap().src(),
                PMDirsImpl::new().unwrap().old(),
            )),
            PMError::Spawn(_) | PMError::Exec => eyre!(e).wrap_err(
                "Had some illegal arguments or problems with io, or failed at building.
Please edit with:
`agpm edit {{project that failed}}`
And then run run:
`agpm update {{all not updated projects}}`",
            ),
            _ => eyre!(e).wrap_err(NO_FIX),
        })?,
        Commands::Restore { project } => pm.i_restore(&project).map_err(|e| match e {
            PMError::Common(CommonPMErrors::NonExisting) => {
                eyre!(e).wrap_err(format!("{} {:?}\n{}", NON_EXIST.0, project, NON_EXIST.1))
            }
            PMError::FileExt(_) => eyre!(e).wrap_err(format!(
                "Error while move files, check the permissions for the directories:
    - {:?}
    - {:?}
and run again.
",
                PMDirsImpl::new().unwrap().old(),
                PMDirsImpl::new().unwrap().src(),
            )),
            PMError::IO(_) => eyre!(e).wrap_err(format!(
                "Error while erasing files, check the permissions in the files:
    - {:?}
and run again.",
                PMDirsImpl::new().unwrap().src(),
            )),
            PMError::Spawn(_) | PMError::Exec => eyre!(e).wrap_err(
                "Had some illegal arguments or problems with io, or failed at building.
Please edit with:
`agpm edit {{project that failed}}`
And then run run:
`agpm restore {{all not restored projects}}`",
            ),
            _ => eyre!(e).wrap_err(NO_FIX),
        })?,
        Commands::Reinstall { project } => pm.reinstall(project).map_err(|e| match e {
            _ => eyre!(e).wrap_err("Running a composed command, can't separate errors"),
        })?,
        Commands::Rebuild { project } => pm.rebuild(&project).map_err(|e| match e {
            PMError::Spawn(_) | PMError::Exec => eyre!(e).wrap_err(format!(
                "Had some illegal arguments or problems with io, or failed at building.
Please edit with:
`agpm edit {project}`
And then run run:
`agpm restore {project}`",
            )),
            PMError::Common(CommonPMErrors::NonExisting) => {
                eyre!(e).wrap_err(format!("{} {:?}\n{}", NON_EXIST.0, project, NON_EXIST.1))
            }
            _ => eyre!(e).wrap_err(NO_FIX),
        })?,
        Commands::List { project } => pm.i_list(&project).map_err(|e| match e {
            PMError::Common(CommonPMErrors::NonExisting) => {
                eyre!(e).wrap_err(format!("{} {:?}\n{}", NON_EXIST.0, project, NON_EXIST.1))
            }
            _ => eyre!(e).wrap_err(NO_FIX),
        })?,
        Commands::Edit { project } => pm.i_edit(&project).map_err(|e| match e{
            PMError::Common(CommonPMErrors::NonExisting) => {
                eyre!(e).wrap_err(format!("{} {:?}\n{}", NON_EXIST.0, project, NON_EXIST.1))
            }
            _ => eyre!(e).wrap_err("There was some error while editing, try again, please")
        })?,
        Commands::Clean => pm.cleanup().map_err(|e| match e {
            PMError::IO(_) => eyre!(e).wrap_err(format!(
                "Error while erasing files, check the permissions for the directories:
    - {:?}
    - {:?}
    - {:?}
and their subdirectories run again.",
                PMDirsImpl::new().unwrap().src(),
                PMDirsImpl::new().unwrap().git(),
                PMDirsImpl::new().unwrap().old(),
            )),
            _ => eyre!(e).wrap_err(NO_FIX),
        })?,
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
        Commands::UpdateSuggestions => agpm_suggestions::download_resources::<PMDirsImpl>()
            .map_err(|e| match e {
                _ => {
                    eyre!(e).wrap_err("Failed to download new suggestions. Please, try again later")
                }
            })?,
    };
    Ok(())
}
