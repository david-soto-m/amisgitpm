use crate::{PMOperations, Project, ProjectStore};

/// A trait whose Defaults are Sane, but bad.
pub trait PMBasics: PMOperations {
    type Store: ProjectStore;
    fn install(&mut self, prj: &Project) -> Result<(), Self::Error>;
    fn uninstall(&mut self, prj_name: &str) -> Result<(), Self::Error>;
    fn update(&self, prj_name: &str) -> Result<(), Self::Error>;
    fn restore(&self, prj_name: &str) -> Result<(), Self::Error>;
    fn edit(&mut self, prj_name: &str, prj: Project) -> Result<(), Self::Error>;
    fn cleanup(&self) -> Result<(), Self::Error>;
}
