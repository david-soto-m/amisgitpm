use crate::Suggester;
use crate::{Project, ProjectStore, UpdatePolicy};
use git2::Repository;
use std::path::Path;

/*
"Now we are trying to establish build instructions. To help with that we have
compiled some suggestions. These come from previous knowledge about build
systems or the README.md file. Assume the commands you leave will start
executing in the root directory of the project."
*/

pub trait Interactions
where
    Self: Sized,
{
    type Suggester: Suggester;
    type Error: std::error::Error + From<git2::Error> + From<<Self::Suggester as Suggester>::Error>;
    fn new() -> Result<Self, Self::Error>;

    fn refs(&self, repo: &Repository) -> Result<String, Self::Error>;
    fn get_sugg(&self, sug: &Vec<Vec<String>>, info: &str) -> Result<Vec<String>, Self::Error>;
    fn get_name_or_dir(
        &self,
        sugg: &str,
        prompts: (&str, &str, &str),
        check: impl Fn(&str) -> bool,
    ) -> Result<String, Self::Error>;

    fn get_updates(&self) -> Result<UpdatePolicy, Self::Error>;
    fn create_project(
        &self,
        path: &Path,
        prj_stub: &Project,
        store: &impl ProjectStore,
    ) -> Result<Project, Self::Error>;

    fn edit(&self, prj: Project) -> Result<Project, Self::Error>;
    fn list<T: ProjectStore>(&self, store: &T) -> Result<(), Self::Error>;
    fn list_one(&self, prj: &Project) -> Result<(), Self::Error>;
    fn update_confirm(&self, package_name: &str) -> Result<bool, Self::Error>;
}
