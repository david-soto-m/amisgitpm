use crate::{
    interaction::Interactions,
    package_management::{CommonError, PackageManagementAtomic, PackageManagementCore},
    projects::{Project, ProjectStore, UpdatePolicy},
};

pub trait PackageManagementInteractive: PackageManagementCore {
    type Interact: Interactions;
    type ErrorI: std::error::Error
        + From<<Self::Store as ProjectStore>::Error>
        + From<std::io::Error>
        + From<CommonError>
        + From<<Self::Interact as Interactions>::Error>
        + From<<Self as PackageManagementAtomic>::Error>;
    fn inter_install(&self, url: &str) -> Result<(), Self::ErrorI> {
        let inter = Self::Interact::new()?;
        let store = Self::Store::new()?;
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
        proj_stub.ref_string = ref_name.to_string();
        self.switch_branch(&proj_stub, repo)?;

        let project = inter.create_project(&git_dir, &proj_stub, &store)?;
        self.build(&project, &git_dir)?;
        std::fs::remove_dir_all(&git_dir)?;
        Ok(())
    }
    fn list(&self, pkg_name: Option<String>) -> Result<(), Self::ErrorI> {
        let inter = Self::Interact::new()?;
        let project_store = Self::Store::new()?;
        match pkg_name {
            Some(pkg) => {
                let project = project_store
                    .get_ref(&pkg)
                    .ok_or(CommonError::NonExisting)?;
                inter.list_one(&pkg, project)?;
                Ok(())
            }
            None => {
                inter.list(&project_store)?;
                Ok(())
            }
        }
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
    fn inter_update(&self, pkg_name: Option<String>, force: bool) -> Result<(), Self::ErrorI> {
        let inter = Self::Interact::new()?;
        let project_store = Self::Store::new()?;
        match pkg_name {
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
