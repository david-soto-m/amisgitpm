use glob::{GlobError, PatternError};
use json_tables::TableError;
use std::fmt::Display;

#[derive(Debug)]
/// An error type for the BuildSuggestions struct.
pub enum SuggestionsError {
    /// The creation has had an error with some file operation
    FileOp(std::io::Error),
    /// The creation has had an error with a json_table
    Table(TableError),
    /// Couldn't read file to determine if it matches pattern
    Glob(GlobError),
    /// Readme Pattern was bad
    Pattern(PatternError),
    /// The path is not utf-8
    Path,
}

impl Display for SuggestionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileOp(e) => write!(f, "{e}"),
            Self::Table(e) => write!(f, "{e}"),
            Self::Glob(e) => write!(f, "{e}"),
            Self::Pattern(e) => write!(f, "{e}"),
            Self::Path => write!(f, "The path provided is not utf-8 compatible"),
        }
    }
}

impl std::error::Error for SuggestionsError {}

impl From<std::io::Error> for SuggestionsError {
    fn from(a: std::io::Error) -> Self {
        Self::FileOp(a)
    }
}

impl From<TableError> for SuggestionsError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

impl From<GlobError> for SuggestionsError {
    fn from(e: GlobError) -> Self {
        Self::Glob(e)
    }
}

impl From<PatternError> for SuggestionsError {
    fn from(e: PatternError) -> Self {
        Self::Pattern(e)
    }
}
