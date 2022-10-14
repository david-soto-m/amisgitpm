use thiserror::Error;
use json_tables::{TableBuilderError, TableError};

#[derive(Debug, Error)]
pub enum ProjectStoreError {
    #[error(transparent)]
    Table(#[from] TableError),
    #[error(transparent)]
    Create(#[from] TableBuilderError),
    #[error(transparent)]
    Other(Box<dyn std::error::Error>),
}
