use std::fmt;

#[derive(Debug)]
pub enum InteractError {
    Git(git2::Error),
    IO(std::io::Error),
    Serde(serde_json::Error),
    Other(String),
}
impl std::error::Error for InteractError {}
impl fmt::Display for InteractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Git(e) => write!(f, "Error while getting the git references: {e}"),
            Self::Other(e) => write!(f, "{e}"),
            Self::Serde(e) => write!(f, "{e}"),
            Self::IO(e) => write!(f, "{e}"),
        }
    }
}
impl From<std::io::Error> for InteractError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
impl From<git2::Error> for InteractError {
    fn from(e: git2::Error) -> Self {
        Self::Git(e)
    }
}

impl From<serde_json::Error> for InteractError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}
