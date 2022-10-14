pub use crate::{
    build_suggestions::SuggestionsError, interaction::InteractError,
    package_management::pm_core::ScriptType, projects::ProjectStoreError,
};

pub const ALREADY_EXISTING: &str = "A project with that name or directory already exists";
pub const EXEC: (&str, &str, &str) = (
    "The",
    "process failed.
Edit the project config with `amisgitpm edit`",
    "Then rebuild with `amisgitpm rebuild",
);
pub const FILE_EXT: &str = "Extended file operations (likely a recursive copy) failed:";
pub const GIT: (&str, &str) = (
    "Had an error on a Git operation with error:",
    "Use the command `amisgitpm cleanup` and try again.",
);
pub const INTERACTION: &str = "Interaction failed:";
pub const IO: &str = "An input/output operation failed:";
pub const NON_EXISTANT: &str = "That project that doesn't exist!
To list all projects use `amisgitpm list`";
pub const OS_2_STR: &str = "Couldn't convert from &Osstr to utf-8 &str";
pub const PROJECTSTORE: &str = "Project store failed:";
pub const SPAWN: (&str, &str) = (
    "Error while spawning the install process:",
    "Try rebuilding the project with `amisgitpm rebuild {{project_name}}`",
);
pub const SUGGESTION: &str = "Suggestions failed:";

#[derive(Debug)]
pub enum PMError {
    AlreadyExisting,
    Exec(String, ScriptType),
    FileExt(fs_extra::error::Error),
    Git(git2::Error),
    Interact(InteractError),
    IO(std::io::Error),
    NonExisting,
    Os2str,
    Other(String),
    ProjectStore(ProjectStoreError),
    Spawn(subprocess::PopenError),
    Suggestions(SuggestionsError),
}

impl std::error::Error for PMError {}

impl std::fmt::Display for PMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExisting => {
                write!(f, "{}", ALREADY_EXISTING)
            }

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
            Self::FileExt(e) => write!(f, "{}\n{e}", FILE_EXT),
            Self::Git(e) => write!(f, "{}\n{e}\n{}", GIT.0, GIT.1),
            Self::Interact(e) => write!(f, "{}\n{e}", INTERACTION),
            Self::IO(e) => write!(f, "{}\n{e}", IO),
            Self::NonExisting => {
                write!(f, "{}", NON_EXISTANT)
            }
            Self::Os2str => write!(f, "{}", OS_2_STR),
            Self::Other(e) => write!(f, "{e}"),
            Self::ProjectStore(e) => write!(f, "{}\n{e}", PROJECTSTORE),
            Self::Spawn(e) => write!(f, "{}\n{e}\n{}", SPAWN.0, SPAWN.1),
            Self::Suggestions(e) => write!(f, "{}\n{e}", SUGGESTION),
        }
    }
}

impl From<git2::Error> for PMError {
    fn from(e: git2::Error) -> Self {
        Self::Git(e)
    }
}

impl From<InteractError> for PMError {
    fn from(e: InteractError) -> Self {
        Self::Interact(e)
    }
}

impl From<std::io::Error> for PMError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<SuggestionsError> for PMError {
    fn from(e: SuggestionsError) -> Self {
        Self::Suggestions(e)
    }
}

impl From<ProjectStoreError> for PMError {
    fn from(e: ProjectStoreError) -> Self {
        Self::ProjectStore(e)
    }
}

impl From<fs_extra::error::Error> for PMError {
    fn from(e: fs_extra::error::Error) -> Self {
        Self::FileExt(e)
    }
}

impl From<subprocess::PopenError> for PMError {
    fn from(e: subprocess::PopenError) -> Self {
        Self::Spawn(e)
    }
}
