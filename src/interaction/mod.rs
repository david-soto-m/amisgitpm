use crate::{
    build_suggestions::BuildSuggester,
    projects::{Project, ProjectTable},
};
use git2::Repository;

mod interact_error;
pub use interact_error::*;

pub trait InstallInteractions {
    type Error: std::error::Error;
    fn initial(url: &str, pr_table: &ProjectTable) -> Result<(String, Project), Self::Error>;
    fn refs(repo: &Repository) -> Result<String, Self::Error>;
    fn finish<T: BuildSuggester>(pr: Project, sug: T) -> Result<Project, Self::Error>;
}

pub trait MinorInteractions {
    type Error: std::error::Error;
    fn edit(prj: &mut Project) -> Result<(), Self::Error>;
    fn list(prj: &ProjectTable) -> Result<(), Self::Error>;
    fn list_one(package_name: &str,prj: &Project) -> Result<(), Self::Error>;
}

pub trait UpdateInteractions {
    type Error: std::error::Error;
    fn confirm(package_name: &str) -> Result<bool, UpdateError>;
}

mod install;
pub use install::InstallInterImpl;

mod minor;
pub use minor::MinorInterImpl;

mod update;
pub use update::UpdateInterImpl;
