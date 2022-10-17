use crate::{
    dirutils::PMDirs,
    package_management::CommonError,
    projects::{Project, ProjectStore},
};
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;
use std::path::{Path, PathBuf};
use subprocess::Exec;
#[derive(Debug)]
pub enum ScriptType {
    IScript,
    UnIScript,
}

pub trait PackageManagementBase
where
    Self: Sized,
{
    type Store: ProjectStore;
    type Dirs: PMDirs;
    type Error: std::error::Error
        + From<<Self::Store as ProjectStore>::Error>
        + From<CommonError>
        + From<git2::Error>
        + From<std::io::Error>
        + From<fs_extra::error::Error>
        + From<subprocess::PopenError>;

    fn new() -> Result<Self, Self::Error>;
    fn download(&self, prj: &Project) -> Result<(Repository, PathBuf), Self::Error> {
        let dirs = Self::Dirs::new();
        let git_dir = dirs.git_dirs().join(&prj.dir);
        let repo = Repository::clone(&prj.url, &git_dir)?;
        Ok((repo, git_dir))
    }
    fn switch_branch(&self, prj: &Project, repo: &Repository) -> Result<(), Self::Error> {
        let (obj, refe) = repo.revparse_ext(&prj.ref_string)?;
        repo.checkout_tree(&obj, None)?;
        if let Some(gref) =  refe {
            repo.set_head(gref.name().unwrap())?;
        }else {
            Err(CommonError::BadRef)?;
        }
        Ok(())
    }
    fn build_rm(&self, prj: &Project, path: &Path) -> Result<(), Self::Error> {
        let dirs = Self::Dirs::new();
        let src_dir = dirs.src_dirs().join(&prj.dir);
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        dir::copy(&path, &src_dir, &opts)?;
        std::fs::remove_dir_all(&path)?;
        self.script_runner(prj, ScriptType::IScript)?;
        Ok(())
    }
    fn script_runner(&self, prj: &Project, scr_run: ScriptType) -> Result<(), Self::Error> {
        let dirs = Self::Dirs::new();
        let src_dir = dirs.src_dirs().join(&prj.dir);
        let script = match scr_run {
            ScriptType::IScript => prj.install_script.join("&&"),
            ScriptType::UnIScript => prj.uninstall_script.join("&&"),
        };
        std::env::set_current_dir(&src_dir)?;
        if !Exec::shell(script).join()?.success() {
            Err(CommonError::Exec(prj.name.to_string(), scr_run))?
        } else {
            Ok(())
        }
    }
}
