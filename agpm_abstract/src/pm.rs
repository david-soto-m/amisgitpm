use crate::{Interactions, PMDirs, Project, ProjectStore, UpdatePolicy};
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
    type Store: ProjectStore;
    type Error: std::error::Error + From<std::io::Error>;
    fn new() -> Result<Self, Self::Error>;
    fn get_ref_from_store(&self, prj_name: &str) -> Result<&Project, Self::Error>;
    fn get_clone_from_store(&self, prj_name: &str) -> Result<Project, Self::Error>;
    fn store_check_dir(&self, dir: &str) -> bool;
    fn store_check_name(&self, name: &str) -> bool;
    fn store_check_unique(&self, name: &str, dir: &str) -> bool;
    fn store_add(&self, prj: &Project) -> Result<(), Self::Error>;
    fn store_remove(&self, prj: &str) -> Result<(), Self::Error>;
    fn store_edit(&self, prj: &str, prj: &Project) -> Result<(), Self::Error>;
    fn src_dir(self) -> PathBuf;
    fn old_dir(self) -> PathBuf;
    fn git_dir(self) -> PathBuf;
    fn download(&self, prj: &Project) -> Result<(Repository, PathBuf), Self::Error>;
    fn switch_branch(&self, prj: &Project, repo: &Repository) -> Result<(), Self::Error>;
    fn build_rm(&self, prj: &Project, path: &Path) -> Result<(), Self::Error>;
    fn script_runner(&self, prj: &Project, scr_run: ScriptType) -> Result<(), Self::Error>;
}

/// A trait whose Defaults are Sane, but bad.
pub trait PMBasics: PMOperations {
    fn install(&mut self, prj: &Project) -> Result<(), Self::Error> {
        if !self.store_check_unique(&prj.name, &prj.dir) {
            Err(Self::Error::AlreadyExisting)?;
        }
        let (repo, git_dir) = self.download(prj)?;
        self.switch_branch(prj, &repo)?;
        self.store_add(&prj)?;
        self.build_rm(prj, &git_dir)?;
        Ok(())
    }
    fn uninstall(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        let project = self
            .get_ref_from_store(prj_name)
            .ok_or(Self::Error::NonExisting)?;
        self.script_runner(project, ScriptType::UnIScript)?;
        let src_dir = self.src_dir().join(&project.dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = self.old_dir().join(&project.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        self.store_remove(prj_name)?;
        Ok(())
    }
    fn update(&self, prj_name: &str) -> Result<(), Self::Error>;
    fn restore(&self, prj_name: &str) -> Result<(), Self::Error>{
        let project = self
            .get_ref_from_store(prj_name)
            .ok_or(Self::Error::NonExisting)?;
        let old_dir = self.old_dir().join(&project.dir);
        let src_dir = self.src_dir().join(&project.dir);
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        std::fs::remove_dir_all(&src_dir)?;
        dir::copy(&old_dir, &src_dir, &opts)?;
        self.script_runner(&project, ScriptType::IScript)?;
        Ok(())
    }
    fn cleanup(&self) -> Result<(), Self::Error> {
        let new_dir = self.git_dir();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir)?;
        }
        let src_dir = self.src_dir();
        if src_dir.exists() {
            std::fs::read_dir(src_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if self.store_check_dir(entry.file_name().to_str().ok_or(Self::Error::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), Self::Error>(())
            })?;
        }
        let old_dir = self.old_dir();
        if old_dir.exists() {
            std::fs::read_dir(&old_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if !self.store_check_dir(entry.file_name().to_str().ok_or(Self::Error::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), Self::Error>(())
            })?;
        }
        Ok(())
    }
}

pub trait PMExtended: PMBasics {
    fn reinstall(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self.get_clone_from_store(prj_name)?;
        self.uninstall(prj_name)?;
        self.install(&prj)?;
        Ok(())
    }
    fn rebuild(&self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self.get_ref_from_store(prj_name)?;
        self.script_runner(&prj, ScriptType::IScript)?;
        Ok(())
    }
    fn bootstrap(&mut self) -> Result<(), Self::Error> {
        let prj = Project {
            name: "amisgitpm".into(),
            dir: "amisgitpm".into(),
            url: "https://github.com/david-soto-m/amisgitpm.git".into(),
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local/".into()],
            uninstall_script: vec!["cargo uninstall amisgitpm --root ~/.local/".into()],
        };
        self.install(&prj)
    }
}
pub trait PMInteractive: PMBasics {
    type Interact: Interactions;
    fn inter_install(&mut self, url: &str) -> Result<(), Self::Error>;
    fn list(&self, prj_names: Vec<String>) -> Result<(), Self::Error>;
    fn inter_edit(&mut self, package: &str) -> Result<(), Self::Error>;
    fn inter_update(&self, prj_name: Option<String>, force: bool) -> Result<(), Self::Error>;
}
