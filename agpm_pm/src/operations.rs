use crate::{PMError, ProjectManager};
use agpm_abstract::*;
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use subprocess::Exec;

impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMOperations for ProjectManager<D, PS, I> {
    type Store = PS;
    type Dirs = D;
    type Error = PMError<I::Error, PS::Error, D::Error>;
    fn new() -> Result<Self, Self::Error> {
        let dirs = D::new().map_err(Self::Error::Dirs)?;
        Ok(Self {
            dirs,
            store: PS::new().map_err(Self::Error::Store)?,
            inter_data: PhantomData::default(),
        })
    }
    fn download(&self, prj: &Project) -> Result<(Repository, PathBuf), Self::Error> {
        let git_dir = self.dirs.git_dirs().join(&prj.dir);
        let repo = Repository::clone(&prj.url, &git_dir)?;
        Ok((repo, git_dir))
    }
    fn switch_branch(&self, prj: &Project, repo: &Repository) -> Result<(), Self::Error> {
        let (obj, refe) = repo.revparse_ext(&prj.ref_string)?;
        repo.checkout_tree(&obj, None)?;
        if let Some(gref) = refe {
            repo.set_head(gref.name().unwrap())?;
        } else {
            Err(Self::Error::BadRef)?;
        }
        Ok(())
    }
    fn build_rm(&self, prj: &Project, path: &Path) -> Result<(), Self::Error> {
        let src_dir = self.dirs.src_dirs().join(&prj.dir);
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        if src_dir.exists() {
            std::fs::remove_dir_all(&src_dir)?;
        }
        // We copy rather than rename due to the platform specific behavior of
        // std::fs::rename
        dir::copy(&path, &src_dir, &opts)?;
        std::fs::remove_dir_all(&path)?;
        self.script_runner(prj, ScriptType::IScript)?;
        Ok(())
    }
    fn script_runner(&self, prj: &Project, scr_run: ScriptType) -> Result<(), Self::Error> {
        let src_dir = self.dirs.src_dirs().join(&prj.dir);
        let script = match scr_run {
            ScriptType::IScript => prj.install_script.join("&&"),
            ScriptType::UnIScript => prj.uninstall_script.join("&&"),
        };
        if !Exec::shell(script).cwd(&src_dir).join()?.success() {
            Err(Self::Error::Exec(prj.name.to_string(), scr_run))?
        } else {
            Ok(())
        }
    }
}
