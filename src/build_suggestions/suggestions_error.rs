use std::fmt::Display;

#[derive(Debug)]
pub enum SuggestionsError {
    FileOpError(std::io::Error),
}

impl Display for SuggestionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Weird error with a Table"),
        }
    }
}

impl std::error::Error for SuggestionsError {}

impl From<std::io::Error> for SuggestionsError {
    fn from(a: std::io::Error) -> Self {
        Self::FileOpError(a)
    }
}