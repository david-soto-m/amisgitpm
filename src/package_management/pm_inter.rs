use crate::{
    interaction::Interactions,
    package_management::{CommonError, PackageManagementBase, PackageManagementCore},
    projects::{Project, ProjectStore, UpdatePolicy},
};

pub trait PackageManagementInteractive: PackageManagementCore {
    type Interact: Interactions;
    type ErrorI: std::error::Error
        + From<<Self::Store as ProjectStore>::Error>
        + From<std::io::Error>
        + From<CommonError>
        + From<<Self::Interact as Interactions>::Error>
        + From<<Self as PackageManagementCore>::ErrorC>
        + From<<Self as PackageManagementBase>::Error>;
    fn inter_install(&self, url: &str) -> Result<(), Self::ErrorI> {
        let url = if url.ends_with('/') {
            let (a, _) = url.rsplit_once('/').unwrap();
            a
        } else {
            url
        };
        let inter = Self::Interact::new()?;
        let mut store = Self::Store::new()?;
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
        let ref_name = inter.refs(&repo)?;
        proj_stub.ref_string = ref_name;
        self.switch_branch(&proj_stub, &repo)?;
        let project = inter.create_project(&git_dir, &proj_stub, &store)?;
        store.add(project.clone())?;
        self.build_rm(&project, &git_dir)?;
        Ok(())
    }
    fn list(&self, prj_names: Vec<String>) -> Result<(), Self::ErrorI> {
        let inter = Self::Interact::new()?;
        let project_store = Self::Store::new()?;
        if prj_names.is_empty() {
            inter.list(&project_store)?;
        } else {
            prj_names.into_iter().try_for_each(|prj_name| {
                let project = project_store
                    .get_ref(&prj_name)
                    .ok_or(CommonError::NonExisting)?;
                inter.list_one(project)?;
                Ok::<_, Self::ErrorI>(())
            })?;
        }
        Ok(())
    }
    fn inter_edit(&self, package: &str) -> Result<(), Self::ErrorI> {
        let inter = Self::Interact::new()?;
        let project_store = Self::Store::new()?;
        if let Some(element) = project_store.get_clone(package) {
            let old_name = element.name.clone();
            let prj = inter.edit(element)?;
            self.edit(&old_name, prj)?;
        }
        Ok(())
    }
    fn inter_update(&self, prj_name: Option<String>, force: bool) -> Result<(), Self::ErrorI> {
        let inter = Self::Interact::new()?;
        let project_store = Self::Store::new()?;
        match prj_name {
            Some(package) => {
                project_store
                    .get_ref(&package)
                    .ok_or(CommonError::NonExisting)?;
                self.update(&package)?;
                Ok(())
            }
            None => {
                project_store
                    .iter()
                    .filter(|e| match e.update_policy {
                        UpdatePolicy::Always => true,
                        UpdatePolicy::Ask => {
                            if !force {
                                inter.update_confirm(&e.name).unwrap_or_default()
                            } else {
                                true
                            }
                        }
                        UpdatePolicy::Never => false,
                    })
                    .try_for_each(|e| self.update(&e.name))?;
                Ok(())
            }
        }
    }
}
