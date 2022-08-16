use std::fmt;

#[derive(Debug)]
pub enum InstallError {
    ConfigError(std::io::Error),
    GitError(git2::Error),
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
        Self::ConfigError(e)
    }
}
impl From<git2::Error> for InstallError {
    fn from(e: git2::Error) -> Self {
        Self::GitError(e)
    }
}

#[derive(Debug)]
pub enum MinorError {
    FileError(std::io::Error),
    SerdeError(serde_json::Error),
}
impl std::error::Error for MinorError {}
impl fmt::Display for MinorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileError(e) => write!(f, "{e}"),
            Self::SerdeError(e) => write!(f, "{e}"),
        }
    }
}
impl From<std::io::Error> for MinorError {
    fn from(e: std::io::Error) -> Self {
        Self::FileError(e)
    }
}
impl From<serde_json::Error> for MinorError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}
