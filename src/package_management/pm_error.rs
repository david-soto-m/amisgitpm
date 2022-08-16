use json_tables::TableError;
use std::fmt;
use subprocess::PopenError;

#[derive(Debug)]
pub enum PMError {
    Common(CommonError),
    Install(InstallError),
    Uninstall(UninstallError),
    Edit(EditError),
    List(ListError),
    Cleanup(CleanupError),
}
impl std::error::Error for PMError {}
impl fmt::Display for PMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Common(e) => write!(f, "{e}"),
            Self::Install(e) => write!(f, "{e}"),
            Self::Uninstall(e) => write!(f, "{e}"),
            Self::Edit(e) => write!(f, "{e}"),
            Self::List(e) => write!(f, "{e}"),
            Self::Cleanup(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug)]
pub enum CommonError {
    Git(git2::Error),
    Table(TableError),
    Path(std::io::Error),
    Process(PopenError),
}
impl std::error::Error for CommonError {}
impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Git(e) => write!(f, "{e}"),
            Self::Table(e) => write!(f, "{e}"),
            Self::Path(e) => write!(f, "{e}"),
            Self::Process(e) => write!(f, "{e}"),
        }
    }
}
impl From<CommonError> for PMError {
    fn from(e: CommonError) -> Self {
        Self::Common(e)
    }
}

#[derive(Debug)]
pub enum InstallError {
    Interact(String),
    Process,
    Suggestions(String),
    Move(std::io::Error),
    AlreadyExisting,
}
impl std::error::Error for InstallError {}
impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interact(e) => write!(f, "{e}"),
            Self::Suggestions(e) => write!(f, "{e}"),
            Self::Process => write!(f, "Failed to build the project"),
            Self::AlreadyExisting => write!(f, "A project with the same name already exists"),
            Self::Move(e) => write!(f, "{e}"),
        }
    }
}
impl From<InstallError> for PMError {
    fn from(e: InstallError) -> Self {
        Self::Install(e)
    }
}

#[derive(Debug)]
pub enum UninstallError {
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
impl From<UninstallError> for PMError {
    fn from(e: UninstallError) -> Self {
        Self::Uninstall(e)
    }
}

#[derive(Debug)]
pub enum EditError {
    Interact(String),
}
impl std::error::Error for EditError {}
impl fmt::Display for EditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interact(e) => write!(f, "{e}"),
        }
    }
}
impl From<EditError> for PMError {
    fn from(e: EditError) -> Self {
        Self::Edit(e)
    }
}

#[derive(Debug)]
pub enum ListError {
    Interact(String),
}
impl std::error::Error for ListError {}
impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interact(e) => write!(f, "{e}"),
        }
    }
}
impl From<ListError> for PMError {
    fn from(e: ListError) -> Self {
        Self::List(e)
    }
}

#[derive(Debug)]
pub enum CleanupError {
    FileOp(std::io::Error),
}
impl std::error::Error for CleanupError {}
impl fmt::Display for CleanupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileOp(e) => write!(f, "{e}"),
        }
    }
}
impl From<CleanupError> for PMError {
    fn from(e: CleanupError) -> Self {
        Self::Cleanup(e)
    }
}


impl From<TableError> for PMError {
    fn from(e: TableError) -> Self {
        Self::Common(CommonError::Table(e))
    }
}

impl From<git2::Error> for PMError {
    fn from(e: git2::Error) -> Self {
        Self::Common(CommonError::Git(e))
    }
}

impl From<PopenError> for PMError {
    fn from(e: PopenError) -> Self {
        Self::Common(CommonError::Process(e))
    }
}
