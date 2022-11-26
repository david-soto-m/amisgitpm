use glob::{GlobError, PatternError};
use json_tables::TableError;
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
/// Interaction failed
pub enum InteractError {
    /// An error while getting the repository refs
    #[error("Error while getting refs: {0}")]
    Git(#[from] git2::Error),
    #[error(transparent)]
    /// An error with an input output operation
    IO(#[from] std::io::Error),
    #[error(transparent)]
    /// Couldn't parse an edited result
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    /// A failure while trying to provide suggestions
    Suggestion(#[from] SuggestionsError),
}

#[non_exhaustive]
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
}
