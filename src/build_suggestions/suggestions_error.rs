use json_tables::TableError;
use std::fmt::Display;
#[derive(Debug)]

/// An error type for the BuildSuggestions struct.
pub enum SuggestionsError {
    /// The creation has had an error with some file operation
    FileOpError(std::io::Error),
    /// The creation has had an error with a json_table
    TableError(TableError),
    /// The path is not utf-8
    PathError,
}

impl Display for SuggestionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileOpError(e) => write!(f, "{e}"),
            Self::TableError(e) => write!(f, "{e}"),
            Self::PathError => write!(f, "The path provided is not utf-8 compatible"),
        }
    }
}

impl std::error::Error for SuggestionsError {}

impl From<std::io::Error> for SuggestionsError {
    fn from(a: std::io::Error) -> Self {
        Self::FileOpError(a)
    }
}

impl From<TableError> for SuggestionsError {
    fn from(e: TableError) -> Self {
        Self::TableError(e)
    }
}
