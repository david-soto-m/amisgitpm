use crate::package_management::pm_error::*;
use json_tables::TableError;

#[derive(Debug)]
pub enum CleanupError {
    Os2str,
    Table(TableError),
    Read(std::io::Error),
    Remove(std::io::Error),
    Other(String)
}

impl std::error::Error for CleanupError {}

impl std::fmt::Display for CleanupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Read(e) => write!(f, "{} {e}", READ),
            Self::Remove(e) => write!(f, "{} {e}", REMOVE),
            Self::Os2str => write!(f, "{}", OS_2_STR),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<TableError> for CleanupError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

#[derive(Debug)]
pub enum ReinstallError {
    NonExistant,
    Install(InstallError),
    Uninstall(UninstallError),
    Table(TableError),
    Other(String),
}

impl std::error::Error for ReinstallError {}

impl std::fmt::Display for ReinstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Install(e) => write!(f, "{e}"),
            Self::NonExistant => write!(f, "{} {} {}", NON_EXISTANT.0, "uninstall", NON_EXISTANT.1),
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Uninstall(e) => write!(f, "{e}"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<InstallError> for ReinstallError {
    fn from(e: InstallError) -> Self {
        Self::Install(e)
    }
}

impl From<UninstallError> for ReinstallError {
    fn from(e: UninstallError) -> Self {
        Self::Uninstall(e)
    }
}

impl From<TableError> for ReinstallError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

#[derive(Debug)]
pub enum RebuildError {
    NonExistant,
    Script(ScriptError),
    Table(TableError),
    Other(String),
}
impl std::error::Error for RebuildError {}
impl std::fmt::Display for RebuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonExistant => write!(f, "{} {} {}", NON_EXISTANT.0, "uninstall", NON_EXISTANT.1),
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Script(e) => write!(f, "{e}"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<TableError> for RebuildError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

impl From<ScriptError> for RebuildError {
    fn from(e: ScriptError) -> Self {
        Self::Script(e)
    }
}

#[derive(Debug)]
pub enum RenameError {
    Table(TableError),
    Other(String),
}
impl std::error::Error for RenameError {}
impl std::fmt::Display for RenameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<TableError> for RenameError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}
