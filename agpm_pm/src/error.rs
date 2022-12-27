use amisgitpm::CommonPMErrors;
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
/// The error type for the `PrjManager` implementations
pub enum PMError<D, ST, I> {
    /// Errors with git operations
    #[error(transparent)]
    Git(#[from] git2::Error),
    /// The error while starting an install/uninstall command
    #[error(transparent)]
    Spawn(#[from] subprocess::PopenError),
    /// An error while recursively copying directories.
    #[error(transparent)]
    FileExt(#[from] fs_extra::error::Error),
    /// An error while opening files, w
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// One of the more common errors
    #[error(transparent)]
    Common(#[from] CommonPMErrors),
    /// An error that has arisen from an interaction
    #[error(transparent)]
    Interact(I),
    /// An error from the store
    #[error(transparent)]
    Store(ST),
    /// An error from the directories
    #[error(transparent)]
    Dirs(D),
    /// An error while executing an install/uninstall command
    #[error("Failed to run script succesfully")]
    Exec,
}
