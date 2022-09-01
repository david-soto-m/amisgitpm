use std::fmt;

#[derive(Debug)]
pub enum InstallError {
    Config(std::io::Error),
    Git(git2::Error),
}
impl std::error::Error for InstallError {}
impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "Weird error with a Table"),
        }
    }
}
impl From<std::io::Error> for InstallError {
    fn from(e: std::io::Error) -> Self {
        Self::Config(e)
    }
}
impl From<git2::Error> for InstallError {
    fn from(e: git2::Error) -> Self {
        Self::Git(e)
    }
}

#[derive(Debug)]
pub enum MinorError {
    File(std::io::Error),
    Serde(serde_json::Error),
}
impl std::error::Error for MinorError {}
impl fmt::Display for MinorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(e) => write!(f, "{e}"),
            Self::Serde(e) => write!(f, "{e}"),
        }
    }
}
impl From<std::io::Error> for MinorError {
    fn from(e: std::io::Error) -> Self {
        Self::File(e)
    }
}
impl From<serde_json::Error> for MinorError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

#[derive(Debug)]
pub enum UpdateError {
    Confirm(std::io::Error),
}
impl std::error::Error for UpdateError {}
impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "here I am"),
        }
    }
}
impl From<std::io::Error> for UpdateError {
    fn from(e: std::io::Error) -> Self {
        Self::Confirm(e)
    }
}
