pub use crate::{
    build_suggestions::SuggestionsError, interaction::InteractError,
    package_management::ScriptType, projects::ProjectStoreError,
};
use thiserror::Error;

pub const GIT: (&str, &str) = (
    "Had an error on a Git operation with error:",
    "Use the command `amisgitpm cleanup` and try again.",
);
pub const SPAWN: (&str, &str) = (
    "Error while spawning the install process:",
    "Try rebuilding the project with `amisgitpm rebuild {{project_name}}`",
);

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PMError {
    #[error("{}\n{0}\n{}", GIT.0, GIT.1)]
    Git(#[from] git2::Error),
    #[error("{}\n{0}\n{}", SPAWN.0, SPAWN.1)]
    Spawn(#[from] subprocess::PopenError),
    #[error(transparent)]
    FileExt(#[from] fs_extra::error::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Commons(#[from] CommonError),
    #[error(transparent)]
    Suggestions(#[from] SuggestionsError),
    #[error(transparent)]
    Interact(#[from] InteractError),
    #[error(transparent)]
    ProjectStore(#[from] ProjectStoreError),
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
Edit the project config with `amisgitpm edit` {0}
Then rebuild with `amisgitpm rebuild{0}",
        match .1 {
            ScriptType::IScript => "install",
            ScriptType::UnIScript => "uninstall",
        },

    )]
    Exec(String, ScriptType),
}
