use crate::{Interactions, PMError, PrjManager};
use amisgitpm::{PMDirs, PMOperations, ProjectStore, ProjectT};
use fs_extra::dir::{self, CopyOptions};
use std::marker::PhantomData;
use std::path::Path;
use subprocess::Exec;

impl<P: ProjectT, D: PMDirs, PS: ProjectStore<P>, I: Interactions<P, PS>> PMOperations
    for PrjManager<P, D, PS, I>
{
    // type Project =;
    type Project = P;
    type Store = PS;
    type Dirs = D;
    type Error = PMError<D::Error, PS::Error, I::Error>;
    fn new() -> Result<Self, Self::Error> {
        let dirs = D::new().map_err(Self::Error::Dirs)?;
        Ok(Self {
            dirs,
            store: PS::new().map_err(Self::Error::Store)?,
            inter_data: PhantomData::default(),
            p_data: PhantomData::default(),
        })
    }
    fn map_store_error(err: <Self::Store as ProjectStore<P>>::Error) -> Self::Error {
        Self::Error::Store(err)
    }
    fn map_dir_error(err: <Self::Dirs as PMDirs>::Error) -> Self::Error {
        Self::Error::Dirs(err)
    }
    fn get_store(&self) -> &Self::Store {
        &self.store
    }
    fn get_mut_store(&mut self) -> &mut Self::Store {
        &mut self.store
    }
    fn get_dirs(&self) -> &Self::Dirs {
        &self.dirs
    }
    fn copy_directory<T: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: T,
        to: Q,
    ) -> Result<(), Self::Error> {
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        dir::copy(from, to, &opts)?;
        Ok(())
    }
    fn script_runner(&self, dir: &str, script: &[String]) -> Result<(), Self::Error> {
        let src_dir = self.dirs.src().join(dir);
        if !Exec::shell(script.join("&&"))
            .cwd(&src_dir)
            .join()?
            .success()
        {
            Err(Self::Error::Exec)?
        }
        Ok(())
    }
}
