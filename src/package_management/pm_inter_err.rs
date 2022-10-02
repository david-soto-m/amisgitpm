use crate::interaction::interact_error::MinorInteractError;
use crate::package_management::pm_error::*;
use json_tables::TableError;

#[derive(Debug)]
pub enum ListError {
    NonExistant,
    Interaction(MinorInteractError),
    Table(TableError),
    Other(String)
}

impl std::error::Error for ListError {}
impl std::fmt::Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonExistant => write!(f, "{} {} {}", NON_EXISTANT.0, "uninstall", NON_EXISTANT.1),
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Interaction(e) => write!(f, "{} {e}", INTERACTION),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<TableError> for ListError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

#[derive(Debug)]
pub enum EditError {
    Table(TableError),
    Interaction(MinorInteractError),
    Other(String)

}

impl std::error::Error for EditError {}
impl std::fmt::Display for EditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Interaction(e) => write!(f, "{} {e}", INTERACTION),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<TableError> for EditError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}
