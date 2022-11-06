use crate::{Project, ProjectStore};
use git2::Repository;
use std::path::Path;

pub trait Interactions
where
    Self: Sized,
{
    type Error: std::error::Error;
    fn new() -> Result<Self, Self::Error>;
    fn refs(&self, repo: &Repository) -> Result<String, Self::Error>;
    fn create_project(
        &self,
        prj_stub: &Project,
        store: &impl ProjectStore,
        wher: &Path,
    ) -> Result<Project, Self::Error>;
    fn edit(&self, prj: Project) -> Result<Project, Self::Error>;
    fn list<T: ProjectStore>(&self, store: &T) -> Result<(), Self::Error>;
    fn list_one(&self, prj: &Project) -> Result<(), Self::Error>;
    fn update_confirm(&self, package_name: &str) -> Result<bool, Self::Error>;
}
