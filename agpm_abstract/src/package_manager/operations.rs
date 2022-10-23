use crate::{PMDirs, Project, ScriptType};
use git2::Repository;
use std::path::{Path, PathBuf};

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
