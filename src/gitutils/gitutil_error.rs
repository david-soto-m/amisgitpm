use crate::interaction::InteractError;
use json_tables::TableError;
use std::fmt;

#[derive(Debug)]
pub enum GitUtilError {
    InteractError(InteractError),
    GitError(git2::Error),
    TableError(TableError),
}

impl fmt::Display for GitUtilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "Weird error with a Table"),
        }
    }
}

impl From<InteractError> for GitUtilError {
    fn from(e: InteractError) -> Self {
        GitUtilError::InteractError(e)
    }
}

impl From<TableError> for GitUtilError {
    fn from(e: TableError) -> Self {
        GitUtilError::TableError(e)
    }
}

impl From<git2::Error> for GitUtilError {
    fn from(e: git2::Error) -> Self {
        GitUtilError::GitError(e)
    }
}
