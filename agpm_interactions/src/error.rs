
use crate::build_suggestions::SuggestionsError;
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
