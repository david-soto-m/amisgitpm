use amisgitpm::{
    PMProgrammatic, PMDirs, PMInteractive, ProjectStore, CommonPMErrors
};
use std::marker::PhantomData;
mod error;
pub use error::PMError;
mod operations;

pub struct ProjectManager<D: PMDirs, PS: ProjectStore, I: Interactions> {
    dirs: D,
    store: PS,
    inter_data: PhantomData<I>,
}
impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMProgrammatic for ProjectManager<D, PS, I> {}


impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMInteractive for ProjectManager<D, PS, I> {
    type Interact = I;

    fn map_inter_error(err: <Self::Interact as Interactions>::Error) -> Self::Error {
        Self::Error::Interact(err)
    }
    fn i_install(&mut self, url: &str) -> Result<(), Self::Error> {
        let url = if url.ends_with('/') {
            let (a, _) = url.rsplit_once('/').unwrap();
            a
        } else {
            url
        };
        let inter = Self::Interact::new().map_err(Self::map_inter_error)?;
        let sugg = url
            .split('/')
            .last()
            .map_or("temp".into(), |potential_dir| {
                potential_dir
                    .to_string()
                    .rsplit_once('.')
                    .map_or(potential_dir.to_string(), |(dir, _)| dir.to_string())
            });
        let mut proj_stub = Project {
            url: url.to_string(),
            dir: sugg,
            ..Default::default()
        };
        let (repo, git_dir) = self.download(&proj_stub)?;
        proj_stub.ref_string = inter.refs(&repo).map_err(Self::map_inter_error)?;
        self.switch_branch(&proj_stub, &repo)?;
        let project = inter
            .create_project(&proj_stub, self.get_store(), &git_dir)
            .map_err(Self::map_inter_error)?;
        self.get_mut_store()
            .add(project.clone())
            .map_err(Self::map_store_error)?;
        self.build_rm(&project, &git_dir)?;
        Ok(())
    }
    fn i_list(&self, prj_names: &[&str]) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::map_inter_error)?;
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
        let inter = Self::Interact::new().map_err(Self::map_inter_error)?;
        if let Some(element) = self.get_store().get_clone(package) {
            let old_name = element.name.clone();
            let prj = inter.edit(element).map_err(Self::map_inter_error)?;
            self.edit(&old_name, prj)?;
        }
        Ok(())
    }
    fn i_update(&self, prj_name: Option<String>) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::map_inter_error)?;
        if let Some(package) = prj_name {
            self.get_store()
                .get_ref(&package)
                .ok_or(CommonPMErrors::NonExisting)?;
            self.update(&package)?;
            Ok(())
        } else {
            self.get_store()
                .iter()
                .filter(|e| match e.update_policy {
                    UpdatePolicy::Always => true,
                    UpdatePolicy::Ask => {
                        inter.update_confirm(&e.name).unwrap_or_default()
                    }
                    UpdatePolicy::Never => false,
                })
                .try_for_each(|e| self.update(&e.name))?;
            Ok(())
        }
    }
}

impl<D: PMDirs, PS: ProjectStore, I: Interactions> ProjectManager<D, PS, I> {
    /// Uninstall a project, and then install it again
    /// Have you tried turning it off and on again?
    fn reinstall(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self
            .get_store()
            .get_clone(prj_name)
            .ok_or(CommonPMErrors::NonExisting)?;
        self.uninstall(prj_name)?;
        self.install(prj)?;
        Ok(())
    }
    /// Run the build script over an existing project.
    fn rebuild(&self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self
            .get_store()
            .get_ref(prj_name)
            .ok_or(CommonPMErrors::NonExisting)?;
        self.script_runner(prj.get_dir(), prj.get_install())?;
        Ok(())
    }
    /// Clean all the files that might be left over from manually touching
    /// config files or unsuccessful uninstallations
    fn cleanup(&self) -> Result<(), Self::Error> {
        let new_dir = self.get_dir().git();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir)?;
        }
        let src_dir = self.get_dir().src();
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
                Ok::<(), Self::Error>(())
            })?;
        }
        let old_dir = self.get_dir().old();
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
                Ok::<(), Self::Error>(())
            })?;
        }
        Ok(())
    }
}
