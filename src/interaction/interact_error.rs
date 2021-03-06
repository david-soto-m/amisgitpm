use std::fmt;

#[derive(Debug)]
pub enum InteractError {
    ConfigError(std::io::Error),
}

impl fmt::Display for InteractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "Weird error with a Table"),
        }
    }
}

impl std::error::Error for InteractError {}

impl From<std::io::Error> for InteractError {
    fn from(e: std::io::Error) -> Self {
        InteractError::ConfigError(e)
    }
}
