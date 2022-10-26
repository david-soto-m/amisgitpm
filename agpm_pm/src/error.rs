use agpm_abstract::{CommonPMErrors, ScriptType};
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PMError<D: std::error::Error, ST: std::error::Error, I: std::error::Error> {
    #[error(
        "Had an error on a Git operation
{0}
If you fix the issue try the command
`amisgitpm rebuild {{project_name}} --from_git`"
    )]
    Git(#[from] git2::Error),
    #[error(
        "Error while spawning the install process:
{0}
Try rebuilding the project with `amisgitpm rebuild {{project_name}}`"
    )]
    Spawn(#[from] subprocess::PopenError),
    #[error(transparent)]
    FileExt(#[from] fs_extra::error::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Common(#[from] CommonPMErrors),
    #[error(transparent)]
    Interact(I),
    #[error(transparent)]
    Store(ST),
    #[error(transparent)]
    Dirs(D),
    #[error("The {} process failed.
Edit the project config with `amisgitpm edit {0}`
Then rebuild with `amisgitpm rebuild {0}`",
        match .1 {
            ScriptType::IScript => "install",
            ScriptType::UnIScript => "uninstall",
        },

    )]
    Exec(String, ScriptType),
}
