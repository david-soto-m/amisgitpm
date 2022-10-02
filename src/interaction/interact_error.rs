use std::fmt;

#[derive(Debug)]
pub enum InstallInteractError {
    Config(std::io::Error),
    Git(git2::Error),
    Other(String),
}
impl std::error::Error for InstallInteractError {}
impl fmt::Display for InstallInteractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(e) => write!(f, "Error while interacting: {e}"),
            Self::Git(e) => write!(f, "Error while getting the git references: {e}"),
            Self::Other(e) => write!(f, "{e}")
        }
    }
}
impl From<std::io::Error> for InstallInteractError {
    fn from(e: std::io::Error) -> Self {
        Self::Config(e)
    }
}
impl From<git2::Error> for InstallInteractError {
    fn from(e: git2::Error) -> Self {
        Self::Git(e)
    }
}

#[derive(Debug)]
pub enum MinorInteractError {
    File(std::io::Error),
    Serde(serde_json::Error),
    Confirm(std::io::Error),
    Other(String),
}
impl std::error::Error for MinorInteractError {}
impl fmt::Display for MinorInteractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(e) => write!(f, "{e}"),
            Self::Serde(e) => write!(f, "{e}"),
            Self::Confirm(e) => write!(f, "{e}"),
            Self::Other(e) => write!(f, "{e}")
        }
    }
}
impl From<serde_json::Error> for MinorInteractError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}
