use json_tables::{TableBuilderError, TableError};
use thiserror::Error;

/// An error for the [`ProjectTable`](crate::ProjectTable)
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ProjectStoreError<D: std::error::Error> {
    /// An error occurred with the tables
    #[error(transparent)]
    Table(#[from] TableError),
    /// An error with the table creating process
    #[error(transparent)]
    Create(#[from] TableBuilderError),
    /// An error while creating directories
    #[error(transparent)]
    Dirs(D),
}