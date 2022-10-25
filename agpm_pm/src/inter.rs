use crate::ProjectManager;
use agpm_abstract::*;

impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMInteractive for ProjectManager<D, PS, I> {
    type Interact = I;
    fn inter_install(&mut self, url: &str) -> Result<(), Self::Error> {
        let url = if url.ends_with('/') {
            let (a, _) = url.rsplit_once('/').unwrap();
            a
        } else {
            url
        };
        let inter = Self::Interact::new().map_err(Self::Error::Interact)?;
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
        let ref_name = inter.refs(&repo).map_err(Self::Error::Interact)?;
        proj_stub.ref_string = ref_name;
        self.switch_branch(&proj_stub, &repo)?;
        let project = inter
            .create_project(&git_dir, &proj_stub, &self.store)
            .map_err(Self::Error::Interact)?;
        self.store.add(&project).map_err(Self::Error::Store)?;
        self.build_rm(&project, &git_dir)?;
        Ok(())
    }
    fn list(&self, prj_names: Vec<String>) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::Error::Interact)?;
        if prj_names.is_empty() {
            inter.list(&self.store).map_err(Self::Error::Interact)?;
        } else {
            prj_names.into_iter().try_for_each(|prj_name| {
                let project = self
                    .store
                    .get_ref(&prj_name)
                    .ok_or(Self::Error::NonExisting)?;
                inter.list_one(project).map_err(Self::Error::Interact)?;
                Ok::<_, Self::Error>(())
            })?;
        }
        Ok(())
    }
    fn inter_edit(&mut self, package: &str) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::Error::Interact)?;
        if let Some(element) = self.store.get_clone(package) {
            let old_name = element.name.clone();
            let prj = inter.edit(element).map_err(Self::Error::Interact)?;
            self.edit(&old_name, prj)?;
        }
        Ok(())
    }
    fn inter_update(&self, prj_name: Option<String>, force: bool) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::Error::Interact)?;
        match prj_name {
            Some(package) => {
                self.store
                    .get_ref(&package)
                    .ok_or(Self::Error::NonExisting)?;
                self.update(&package)?;
                Ok(())
            }
            None => {
                self.store
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
