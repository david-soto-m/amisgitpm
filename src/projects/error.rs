use json_tables::{TableBuilderError, TableError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectStoreError {
    #[error(transparent)]
    Table(#[from] TableError),
    #[error(transparent)]
    Create(#[from] TableBuilderError),
}
