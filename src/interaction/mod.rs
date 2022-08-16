use crate::{
    build_suggestions::BuildSuggester,
    projects::{Project, ProjectTable},
};
use git2::Repository;

mod interact_error;
pub use interact_error::InteractError;

pub trait InstallInteractions {
    type Error: std::error::Error;
    fn initial(url: &str, pr_table: &ProjectTable) -> Result<Project, Self::Error>;
    fn refs(repo: &Repository) -> Result<String, Self::Error>;
    fn finish<T: BuildSuggester>(pr: Project, sug: T) -> Result<Project, Self::Error>;
}

mod install;
pub use install::UserInstallInteractions;
