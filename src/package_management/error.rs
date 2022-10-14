use thiserror::Error;
pub use crate::{
    build_suggestions::SuggestionsError, interaction::InteractError,
    package_management::ScriptType, projects::ProjectStoreError,
};

#[derive(Debug, Error)]
pub enum PMError{
    #[error("A project with that name or directory already exists")]
    AlreadyExisting,
    #[error("That project that doesn't exist!
To list all projects use `amisgitpm list`")]
    NonExisting,
    #[error("Couldn't convert from &Osstr to utf-8 &str")]
    Os2str,
    #[error(" The {} process failed.\nEdit the project config with `amisgitpm edit`{0}`
Then rebuild with `amisgitpm rebuild {0}`",
            match .1 {
                ScriptType::IScript => "install",
                ScriptType::UnIScript => "uninstall",
            }
    )]
    Exec(String, ScriptType),
    #[error(transparent)]
    FileExt(#[from] fs_extra::error::Error),
    #[error(transparent)]
    Git(#[from] git2::Error),
    #[error(transparent)]
    IO(#[from]std::io::Error),
    #[error(transparent)]
    ProjectStore(#[from] ProjectStoreError),
    #[error(transparent)]
    Interact(#[from] InteractError),
    #[error(transparent)]
    Suggestions(#[from] SuggestionsError),
    #[error(transparent)]
    PopenError(#[from] subprocess::PopenError),
    #[error(transparent)]
    Other(Box<dyn std::error::Error>),
}

