use amisgitpm::CommonPMErrors;
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PMError<D: std::error::Error, ST: std::error::Error, I: std::error::Error> {
    #[error(transparent)]
    Git(#[from] git2::Error),
    #[error(transparent)]
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
    #[error("Failed to run script succesfully")]
    Exec,
}
