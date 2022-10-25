use agpm_abstract::ScriptType;
use std::path::PathBuf;
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PMError<I: std::error::Error, ST: std::error::Error, D: std::error::Error> {
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
    Interact(I),
    #[error(transparent)]
    Store(ST),
    #[error(transparent)]
    Dirs(D),
    #[error("A project with that name or directory already exists")]
    AlreadyExisting,
    #[error(
        "That project that doesn't exist!
To list all projects use `amisgitpm list`"
    )]
    NonExisting,
    #[error("Couldn't convert from &Osstr to utf-8 &str")]
    Os2str,
    #[error("The {} process failed.
Edit the project config with `amisgitpm edit {0}`
Then rebuild with `amisgitpm rebuild {0}`",
        match .1 {
            ScriptType::IScript => "install",
            ScriptType::UnIScript => "uninstall",
        },

    )]
    Exec(String, ScriptType),
    #[error(
        "Update couldn't be solved by a fast forward.
Solve the git problems in {0} and then run `amisgitpm rebuild {1} --from-git"
    )]
    ImposibleUpdate(PathBuf, String),
    #[error("Couldn't find a reference to a non detached HEAD")]
    BadRef,
}
