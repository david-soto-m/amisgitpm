use crate::{
    build_suggestions::BuildSuggester,
    projects::{Project, ProjectTable},
};
use git2::Repository;

mod interact_error;
pub use interact_error::{InstallError, MinorError};

pub trait InstallInteractions {
    type Error: std::error::Error;
    fn initial(url: &str, pr_table: &ProjectTable) -> Result<Project, Self::Error>;
    fn refs(repo: &Repository) -> Result<String, Self::Error>;
    fn finish<T: BuildSuggester>(pr: Project, sug: T) -> Result<Project, Self::Error>;
}

pub trait MinorInteractions {
    type Error: std::error::Error;
    fn edit(prj: &mut Project) -> Result<(), Self::Error>;
    fn list(prj: &ProjectTable) -> Result<(), Self::Error>;
}

mod install;
pub use install::UserInstallInteractions;

mod minor;
pub use minor::MinorInteractionsImpl;
