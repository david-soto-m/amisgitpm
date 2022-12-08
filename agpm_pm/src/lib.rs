use amisgitpm::{
    CommonPMErrors, PMDirs, PMInteractive, PMOperations, PMProgrammatic, ProjectStore, ProjectT,
};
use std::marker::PhantomData;
mod error;
pub use error::PMError;
mod interactions;
mod operations;
pub use interactions::Interactions;

pub struct PrjManager<P: ProjectT, D: PMDirs, PS: ProjectStore<P>, I: Interactions<P, PS>> {
    dirs: D,
    store: PS,
    inter_data: PhantomData<I>,
    p_data: PhantomData<P>,
}

impl<P: ProjectT, D: PMDirs, PS: ProjectStore<P>, I: Interactions<P, PS>> PMProgrammatic
    for PrjManager<P, D, PS, I>
{
}

impl<P: ProjectT, D: PMDirs, PS: ProjectStore<P>, I: Interactions<P, PS>> PMInteractive
    for PrjManager<P, D, PS, I>
{
    fn i_install(&mut self, url: &str) -> Result<(), Self::Error> {
        let inter = I::new().map_err(Self::map_inter_error)?;
        let prj_stub = inter.url_to_download(url).map_err(Self::map_inter_error)?;
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
        self.mv_build(&project, &git_dir)?;
        Ok(())
    }
    fn i_list(&self, prj_names: &[&str]) -> Result<(), Self::Error> {
        let inter = I::new().map_err(Self::map_inter_error)?;
        if prj_names.is_empty() {
            inter
                .list(self.get_store())
                .map_err(Self::map_inter_error)?;
        } else {
            prj_names.into_iter().try_for_each(|prj_name| {
                let project = self
                    .get_store()
                    .get_ref(&prj_name)
                    .ok_or(CommonPMErrors::NonExisting)?;
                inter.list_one(project).map_err(Self::map_inter_error)?;
                Ok::<_, Self::Error>(())
            })?;
        }
        Ok(())
    }
    fn i_edit(&mut self, package: &str) -> Result<(), Self::Error> {
        let inter = I::new().map_err(Self::map_inter_error)?;
        if let Some(element) = self.get_store().get_clone(package) {
            let old_name = element.get_name().to_owned();
            let prj = inter.edit(element).map_err(Self::map_inter_error)?;
            self.edit(&old_name, prj)?;
        }
        Ok(())
    }
    fn i_update(&self, prj_names: &[&str]) -> Result<(), Self::Error> {
        let inter = I::new().map_err(Self::map_inter_error)?;
        if prj_names.is_empty() {
            self.get_store()
                .iter()
                .filter(|e| inter.update_confirm(e))
                .try_for_each(|e| self.update(&e.get_name()))?;
        } else {
            for project in prj_names {
                self.get_store()
                    .get_ref(&project)
                    .ok_or(CommonPMErrors::NonExisting)?;
                self.update(&project)?;
            }
        }
        Ok(())
    }
    fn i_restore(self, prj_names: &[&str]) -> Result<(), Self::Error> {
        for prj in prj_names {
            self.restore(prj)?
        }
        Ok(())
    }
    fn i_uninstall(&mut self, prj_names: &[&str]) -> Result<(), Self::Error> {
        for prj in prj_names {
            self.uninstall(prj)?
        }
        Ok(())
    }
}

impl<P: ProjectT, D: PMDirs, PS: ProjectStore<P>, I: Interactions<P, PS>> PrjManager<P, D, PS, I> {
    fn map_inter_error(e: I::Error) -> <Self as PMOperations>::Error {
        <Self as PMOperations>::Error::Interact(e)
    }

    /// Uninstall a project, and then install it again
    /// Have you tried turning it off and on again?
    pub fn reinstall(&mut self, prj_name: &str) -> Result<(), <Self as PMOperations>::Error> {
        let prj = self
            .get_store()
            .get_clone(prj_name)
            .ok_or(CommonPMErrors::NonExisting)?;
        self.uninstall(prj_name)?;
        self.install(prj)?;
        Ok(())
    }
    /// Run the build script over an existing project.
    pub fn rebuild(&self, prj_name: &str) -> Result<(), <Self as PMOperations>::Error> {
        let prj = self
            .get_store()
            .get_ref(prj_name)
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
