#[cfg(feature = "suggestions")]
use agpm_suggestions::SuggestionsError;
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
    #[cfg(feature = "suggestions")]
    #[error(transparent)]
    /// A failure while trying to provide suggestions
    Suggestion(#[from] SuggestionsError),
}
