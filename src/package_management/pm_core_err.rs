use crate::{
    build_suggestions::SuggestionsError,
    interaction::interact_error::InstallInteractError,
    package_management::{pm_core::ScriptType, pm_error::*},
};
use json_tables::TableError;
use subprocess::PopenError;

#[derive(Debug)]
pub enum ScriptError {
    Path(std::io::Error),
    Spawn(PopenError),
    Exec(String, ScriptType),
    Other(String),
}
impl std::error::Error for ScriptError {}
impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spawn(e) => write!(f, "{} {e}{}", SPAWN.0, SPAWN.1),
            Self::Path(e) => write!(f, "{} {e}", PATH),
            Self::Exec(name, scr) => write!(
                f,
                "{} {} {} {name}` {} {name}`",
                EXEC.0,
                match scr {
                    ScriptType::IScript => "install",
                    ScriptType::UnIScript => "uninstall",
                },
                EXEC.1,
                EXEC.2
            ),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl From<PopenError> for ScriptError {
    fn from(e: PopenError) -> Self {
        Self::Spawn(e)
    }
}

#[derive(Debug)]
pub enum InstallError {
    AlreadyExisting,
    Git(git2::Error),
    Interaction(InstallInteractError),
    Move(std::io::Error),
    Script(ScriptError),
    Suggestions(SuggestionsError),
    Table(TableError),
    Other(String),
}

impl std::error::Error for InstallError {}

impl std::fmt::Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExisting => {
                write!(f, "{}", ALREADY_EXISTING)
            }
            Self::Interaction(e) => write!(f, "{e}"),
            Self::Git(e) => write!(f, "{} {e}{}", GIT.0, GIT.1),
            Self::Move(e) => write!(f, "{} {e}", MOVE),
            Self::Script(e) => write!(f, "{e}"),
            Self::Suggestions(e) => write!(f, "{e}"),
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<TableError> for InstallError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

impl From<ScriptError> for InstallError {
    fn from(e: ScriptError) -> Self {
        Self::Script(e)
    }
}

impl From<git2::Error> for InstallError {
    fn from(e: git2::Error) -> Self {
        Self::Git(e)
    }
}

#[derive(Debug)]
pub enum UninstallError {
    NonExistant,
    Remove(std::io::Error),
    Script(ScriptError),
    Table(TableError),
    Other(String),
}

impl std::error::Error for UninstallError {}
impl std::fmt::Display for UninstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonExistant => write!(f, "{} {} {}", NON_EXISTANT.0, "uninstall", NON_EXISTANT.1),
            Self::Remove(e) => write!(f, "{} {e}", REMOVE),
            Self::Script(e) => write!(f, "{e}"),
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<TableError> for UninstallError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

impl From<ScriptError> for UninstallError {
    fn from(e: ScriptError) -> Self {
        Self::Script(e)
    }
}

#[derive(Debug)]
pub enum RestoreError {
    Copy(fs_extra::error::Error),
    NonExistant,
    Remove(std::io::Error),
    Script(ScriptError),
    Table(TableError),
    Other(String),
}

impl std::error::Error for RestoreError {}

impl std::fmt::Display for RestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Copy(e) => write!(f, "{} {e}", COPY),
            Self::NonExistant => write!(f, "{} {} {}", NON_EXISTANT.0, "restore", NON_EXISTANT.1),
            Self::Remove(e) => write!(f, "{} {e}", REMOVE),
            Self::Script(e) => write!(f, "{e}"),
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Other(e) => write!(f, "{e}"),
            // Self:: => write!(f, ""),
        }
    }
}

impl From<TableError> for RestoreError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

impl From<ScriptError> for RestoreError {
    fn from(e: ScriptError) -> Self {
        Self::Script(e)
    }
}

#[derive(Debug)]
pub enum UpdateError {
    Copy(fs_extra::error::Error),
    NonExistant,
    Script(ScriptError),
    Table(TableError),
    Other(String),
}

impl std::error::Error for UpdateError {}

impl std::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Copy(e) => write!(f, "{} {e}", COPY),
            Self::NonExistant => write!(f, "{} {} {}", NON_EXISTANT.0, "restore", NON_EXISTANT.1),
            Self::Script(e) => write!(f, "{e}"),
            Self::Table(e) => write!(f, "{} {e}", TABLE),
            Self::Other(e) => write!(f, "{e}"),
            // Self:: => write!(f, ""),
        }
    }
}

impl From<TableError> for UpdateError {
    fn from(e: TableError) -> Self {
        Self::Table(e)
    }
}

impl From<ScriptError> for UpdateError {
    fn from(e: ScriptError) -> Self {
        Self::Script(e)
    }
}
