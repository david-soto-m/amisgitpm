use crate::{PMDirs, Project, ProjectStore, Interactions};
use git2::Repository;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ScriptType {
    IScript,
    UnIScript,
}

pub trait PMOperations
where
    Self: Sized,
{
    type Dirs: PMDirs;
    type Error: std::error::Error + 'static;
    fn new() -> Result<Self, Self::Error>;
    fn download(&self, prj: &Project) -> Result<(Repository, PathBuf), Self::Error>;
    fn switch_branch(&self, prj: &Project, repo: &Repository) -> Result<(), Self::Error>;
    fn build_rm(&self, prj: &Project, path: &Path) -> Result<(), Self::Error>;
    fn script_runner(&self, prj: &Project, scr_run: ScriptType) -> Result<(), Self::Error>;
}

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

pub trait PMExtended: PMBasics {
    fn reinstall(&mut self, prj_name: &str) -> Result<(), Self::Error>;
    fn rebuild(&self, prj_name: &str) -> Result<(), Self::Error>;
    fn bootstrap(&mut self) -> Result<(), Self::Error>;
}
pub trait PMInteractive: PMBasics {
    type Interact: Interactions;
    fn inter_install(&mut self, url: &str) -> Result<(), Self::Error>;
    fn list(&self, prj_names: Vec<String>) -> Result<(), Self::Error>;
    fn inter_edit(&mut self, package: &str) -> Result<(), Self::Error>;
    fn inter_update(&self, prj_name: Option<String>, force: bool) -> Result<(), Self::Error>;
}

