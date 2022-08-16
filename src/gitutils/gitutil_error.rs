use json_tables::TableError;
use std::fmt;
use subprocess::PopenError;

#[derive(Debug)]
pub enum GitUtilError {
    Common(CommonError),
    Install(InstallError),
    Uninstall(UninstallError),
}
impl std::error::Error for GitUtilError {}
impl fmt::Display for GitUtilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Common(e) => write!(f, "{e}"),
            Self::Install(e) => write!(f, "{e}"),
            Self::Uninstall(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug)]
pub enum CommonError{
    Interact(String),
    Git(git2::Error),
    Table(TableError),
    Path(std::io::Error),
    Process(PopenError),
}
impl std::error::Error for CommonError {}
impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interact(e) => write!(f, "{e}"),
            Self::Git(e) => write!(f, "{e}"),
            Self::Table(e) => write!(f, "{e}"),
            Self::Path(e) => write!(f, "{e}"),
            Self::Process(e) => write!(f, "{e}"),
        }
    }
}
impl From<CommonError> for GitUtilError{
    fn from(e: CommonError) -> Self {
        Self::Common(e)
    }
}

#[derive(Debug)]
pub enum InstallError{
    Process,
    Suggestions(String),
    Move(std::io::Error),

}
impl std::error::Error for InstallError {}
impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Suggestions(e) => write!(f, "{e}"),
            Self::Process => write!(f, "Failed to build the project"),
            Self::Move(e) => write!(f, "{e}"),
        }
    }
}
impl From<InstallError> for GitUtilError{
    fn from(e: InstallError) -> Self {
        Self::Install(e)
    }
}

#[derive(Debug)]
pub enum UninstallError{
    Process,
    Remove(std::io::Error),
    NonExistant,
}
impl std::error::Error for UninstallError {}
impl fmt::Display for UninstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Process => write!(f, "Failed to uninstall the project"),
            Self::NonExistant => write!(f, "The project does not exist"),
            Self::Remove(e) => write!(f, "{e}"),
        }
    }
}

impl From<UninstallError> for GitUtilError{
    fn from(e: UninstallError) -> Self {
        Self::Uninstall(e)
    }
}




impl From<TableError> for GitUtilError {
    fn from(e: TableError) -> Self {
        GitUtilError::Common(CommonError::Table(e))
    }
}

impl From<git2::Error> for GitUtilError {
    fn from(e: git2::Error) -> Self {
        GitUtilError::Common(CommonError::Git(e))
    }
}

impl From<PopenError> for GitUtilError {
    fn from(e: PopenError) -> Self {
        Self::Common(CommonError::Process(e))
    }
}
