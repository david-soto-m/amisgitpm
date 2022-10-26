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
    type Error = PMError<D::Error, PS::Error, I::Error>;
    fn new() -> Result<Self, Self::Error> {
        let dirs = D::new().map_err(Self::Error::Dirs)?;
        Ok(Self {
            dirs,
            store: PS::new().map_err(Self::Error::Store)?,
            inter_data: PhantomData::default(),
        })
    }
    fn map_store_error(err: <Self::Store as ProjectStore>::Error)->Self::Error {
        Self::Error::Store(err)
    }
    fn map_dir_error(err: <Self::Dirs as PMDirs>::Error)->Self::Error {
        Self::Error::Dirs(err)
    }
    fn get_store(&self) -> &Self::Store {
        &self.store
    }
    fn get_mut_store(&mut self) -> &mut Self::Store {
        &mut self.store
    }
    fn get_dir(&self) -> &Self::Dirs {
        & self.dirs
    }
    fn copy_directory<T: AsRef<Path>, Q: AsRef<Path>>(&self, from: T, to: Q) -> Result<(), Self::Error> {
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        dir::copy(from, to, &opts)?;
        Ok(())
    }
    fn script_runner(&self, prj: &Project, scr_run: ScriptType) -> Result<(), Self::Error> {
        let src_dir = self.dirs.src().join(&prj.dir);
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
