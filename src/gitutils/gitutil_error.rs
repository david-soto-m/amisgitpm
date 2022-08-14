use json_tables::TableError;
use std::fmt;

#[derive(Debug)]
pub enum GitUtilError {
    Interact(String),
    Git(git2::Error),
    Table(TableError),
    Suggestions(String),
}

impl std::error::Error for GitUtilError {}

impl fmt::Display for GitUtilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "Weird error with a Table"),
        }
    }
}

impl From<TableError> for GitUtilError {
    fn from(e: TableError) -> Self {
        GitUtilError::Table(e)
    }
}

impl From<git2::Error> for GitUtilError {
    fn from(e: git2::Error) -> Self {
        GitUtilError::Git(e)
    }
}
