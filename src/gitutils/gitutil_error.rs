use json_tables::TableError;
use std::fmt;
use subprocess::PopenError;

#[derive(Debug)]
pub enum GitUtilError {
    Interact(String),
    Git(git2::Error),
    Table(TableError),
    Suggestions(String),
    Path(std::io::Error),
    Process(PopenError),
    BuildProcess
}

impl std::error::Error for GitUtilError {}

impl fmt::Display for GitUtilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interact(e) => write!(f, "{e}"),
            Self::Git(e) => write!(f, "{e}"),
            Self::Table(e) => write!(f, "{e}"),
            Self::Suggestions(e) => write!(f, "{e}"),
            Self::Path(e)=> write!(f, "{e}"),
            Self::Process(e)=> write!(f, "{e}"),
            Self::BuildProcess=> write!(f, "Failed to build the project"),

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

impl From<PopenError> for GitUtilError {
    fn from(e: PopenError) -> Self {
        Self::Process(e)
    }
}
