use glob::{GlobError, PatternError};
use json_tables::TableError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InteractError {
    #[error("Error while getting refs: {0}")]
    Git(#[from] git2::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Suggestion(#[from] SuggestionsError),
    #[error(transparent)]
    Other(Box<dyn std::error::Error>),
}

#[derive(Error, Debug)]
/// An error type for the `BuildSuggestions` struct.
pub enum SuggestionsError {
    /// The creation has had an error with some file operation
    #[error(transparent)]
    FileOp(#[from] std::io::Error),
    /// The creation has had an error with a json_table
    #[error(transparent)]
    Table(#[from] TableError),
    /// Couldn't read file to determine if it matches pattern
    #[error(transparent)]
    Glob(#[from] GlobError),
    /// A glob pattern was bad
    #[error(transparent)]
    Pattern(#[from] PatternError),
    /// The path is not utf-8
    #[error("A path is not utf-8 compatible")]
    Path,

    /// A field to place errors that don't fit in with the other variants when
    /// re-implementing the BuildSuggestions
    #[error("{0}")]
    DirsError(String),
    /// A field to place errors that don't fit in with the other variants when
    /// re-implementing the BuildSuggestions
    #[error("{0}")]
    Other(String),
}
