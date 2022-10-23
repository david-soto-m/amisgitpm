use thiserror::Error;
use std::path::PathBuf;

mod operations;
pub use operations::*;
mod base;
pub use base::*;
mod extended;
pub use extended::*;
mod inter;
pub use inter::*;
#[derive(Debug)]
pub enum ScriptType {
    IScript,
    UnIScript,
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum CommonError {
    #[error("A project with that name or directory already exists")]
    AlreadyExisting,
    #[error(
        "That project that doesn't exist!
To list all projects use `amisgitpm list`"
    )]
    NonExisting,
    #[error("Couldn't convert from &Osstr to utf-8 &str")]
    Os2str,
    #[error(transparent)]
    Other(Box<dyn std::error::Error>),
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
