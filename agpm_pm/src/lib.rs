#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use amisgitpm::{
    CommonPMErrors, Directories, PMInteractive, PMOperations, PMProgrammatic, ProjectIface,
    ProjectStore,
};
use std::marker::PhantomData;
mod error;
pub use error::PMError;
mod interactions;
mod operations;
pub use interactions::Interactions;

/// The implementor of the three project manager traits `PMOperations`, `PMInteractive` and `PMProgrammatic`
///
/// It's generic over the `ProjectIface`, `ProjectStore`, the `Directories` and `Interactions`.
/// This way the it can be used in other code, just changing the `Interactions` implementor you
/// can swap all the pieces and still get a functional `PrjManager`
pub struct PrjManager<P: ProjectIface, D: Directories, PS: ProjectStore<P>, I: Interactions<P, PS>>
{
    dirs: D,
    store: PS,
    inter_data: PhantomData<I>,
    p_data: PhantomData<P>,
}

impl<P: ProjectIface, D: Directories, PS: ProjectStore<P>, I: Interactions<P, PS>> PMProgrammatic
    for PrjManager<P, D, PS, I>
{
}
impl<P: ProjectIface, D: Directories, PS: ProjectStore<P>, I: Interactions<P, PS>> PMInteractive
    for PrjManager<P, D, PS, I>
{
    fn i_install<T: AsRef<str>>(&mut self, url: T) -> Result<(), Self::Error> {
        let inter = I::new().map_err(Self::map_inter_error)?;
        let prj_stub = inter
            .url_to_download(url.as_ref())
            .map_err(Self::map_inter_error)?;
        let (repo, git_dir) = self.download(&prj_stub)?;
        let prj_stub = inter
            .repo_to_checkout_branch(prj_stub, &repo)
            .map_err(Self::map_inter_error)?;
        self.switch_branch(&prj_stub, &repo)?;
        let project = inter
            .create_project(&prj_stub, self.get_store(), &git_dir)
            .map_err(Self::map_inter_error)?;
        self.get_mut_store()
            .add(project.clone())
            .map_err(Self::map_store_error)?;
        self.mv(&project, &git_dir)?;
        self.build(&project)?;
        Ok(())
    }
    fn i_list<T: AsRef<str>, Q: AsRef<[T]>>(&self, prj_names: Q) -> Result<(), Self::Error> {
        let inter = I::new().map_err(Self::map_inter_error)?;
        if prj_names.as_ref().is_empty() {
            inter
                .list(self.get_store())
                .map_err(Self::map_inter_error)?;
        } else {
            prj_names.as_ref().iter().try_for_each(|prj_name| {
                let project = self
                    .get_store()
                    .get_ref(prj_name.as_ref())
                    .ok_or(CommonPMErrors::NonExisting)?;
                inter.list_one(project).map_err(Self::map_inter_error)?;
                Ok::<_, Self::Error>(())
            })?;
        }
        Ok(())
    }
    fn i_edit<T: AsRef<str>>(&mut self, project: T) -> Result<(), Self::Error> {
        let inter = I::new().map_err(Self::map_inter_error)?;
        if let Some(element) = self.get_store().get_clone(project.as_ref()) {
            let old_name = element.get_name().to_owned();
            let prj = inter.edit(element).map_err(Self::map_inter_error)?;
            self.edit(&old_name, prj)?;
        }
        Ok(())
    }
    fn i_update<T: AsRef<str>, Q: AsRef<[T]>>(&self, prj_names: Q) -> Result<(), Self::Error> {
        let inter = I::new().map_err(Self::map_inter_error)?;
        if prj_names.as_ref().is_empty() {
            self.get_store()
                .iter()
                .filter(|e| inter.update_confirm(e))
                .try_for_each(|e| self.update(e.get_name()))?;
        } else {
            for project in prj_names.as_ref() {
                self.update(project.as_ref())?;
            }
        }
        Ok(())
    }
    fn i_restore<T: AsRef<str>, Q: AsRef<[T]>>(self, prj_names: Q) -> Result<(), Self::Error> {
        for prj in prj_names.as_ref() {
            self.restore(prj)?;
        }
        Ok(())
    }
    fn i_uninstall<T: AsRef<str>, Q: AsRef<[T]>>(
        &mut self,
        prj_names: Q,
    ) -> Result<(), Self::Error> {
        for prj in prj_names.as_ref() {
            self.uninstall(prj)?;
        }
        Ok(())
    }
}

impl<P: ProjectIface, D: Directories, PS: ProjectStore<P>, I: Interactions<P, PS>>
    PrjManager<P, D, PS, I>
{
    fn map_inter_error(e: I::Error) -> <Self as PMOperations>::Error {
        <Self as PMOperations>::Error::Interact(e)
    }

    /// Uninstall a project, and then install it again
    /// Have you tried turning it off and on again?
    pub fn reinstall<T: AsRef<str>>(
        &mut self,
        prj_name: T,
    ) -> Result<(), <Self as PMOperations>::Error> {
        let prj = self
            .get_store()
            .get_clone(prj_name.as_ref())
            .ok_or(CommonPMErrors::NonExisting)?;
        self.uninstall(prj_name)?;
        self.install(prj)?;
        Ok(())
    }
    /// Run the build script over an existing project.
    pub fn rebuild<T: AsRef<str>>(&self, prj_name: T) -> Result<(), <Self as PMOperations>::Error> {
        let prj = self
            .get_store()
            .get_ref(prj_name.as_ref())
            .ok_or(CommonPMErrors::NonExisting)?;
        self.script_runner(prj.get_dir(), prj.get_install())?;
        Ok(())
    }
    /// Clean all the files that might be left over from manually touching
    /// config files or unsuccessful uninstallations
    pub fn cleanup(&self) -> Result<(), <Self as PMOperations>::Error> {
        let new_dir = self.get_dirs().git();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir)?;
        }
        let src_dir = self.get_dirs().src();
        if src_dir.exists() {
            std::fs::read_dir(src_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if self
                        .get_store()
                        .check_dir_free(entry.file_name().to_str().ok_or(CommonPMErrors::Os2Str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), <Self as PMOperations>::Error>(())
            })?;
        }
        let old_dir = self.get_dirs().old();
        if old_dir.exists() {
            std::fs::read_dir(&old_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if !self
                        .get_store()
                        .check_dir_free(entry.file_name().to_str().ok_or(CommonPMErrors::Os2Str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), <Self as PMOperations>::Error>(())
            })?;
        }
        Ok(())
    }
}
