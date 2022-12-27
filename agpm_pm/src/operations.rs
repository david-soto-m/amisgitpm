use crate::{Interactions, PMError, PrjManager};
use amisgitpm::{Directories, PMOperations, ProjectIface, ProjectStore};
use fs_extra::dir::{self, CopyOptions};
use std::marker::PhantomData;
use std::path::Path;
use subprocess::Exec;

impl<P: ProjectIface, D: Directories, PS: ProjectStore<P>, I: Interactions<P, PS>> PMOperations
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
    fn map_dir_error(err: <Self::Dirs as Directories>::Error) -> Self::Error {
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
        dir::copy(from, to, &opts).map_err(|e|{ println!("{e}"); e})?;
        Ok(())
    }
    fn script_runner<T: AsRef<str>, Q: AsRef<[T]>>(
        &self,
        dir: &str,
        script: Q,
    ) -> Result<(), Self::Error> {
        let src_dir = self.dirs.src().join(dir);
        let script: Vec<&str> = script.as_ref().iter().map(|e| e.as_ref()).collect();
        if !Exec::shell(script.join("&&"))
            .cwd(src_dir)
            .join()?
            .success()
        {
            Err(Self::Error::Exec)?;
        }
        Ok(())
    }
}
