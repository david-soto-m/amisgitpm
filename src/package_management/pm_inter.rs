use crate::{
    dirutils::PMDirs,
    interaction::{InstallInteractions, MinorInteractions},
    package_management::{PMError, PackageManagementCore, ScriptType},
    projects::{ProjectStore, UpdatePolicy},
};
use git2::Repository;

pub trait PackageManagementInteractive: PackageManagementCore {
    fn inter_install<T>(&self, url: &str, inter: T) -> Result<(), PMError>
    where
        T: InstallInteractions,
    {
        let dirs = Self::Dirs::new();
        let mut project_store = Self::Store::new()?;
        let mut proj_stub = inter.initial(url, &project_store)?;
        let new_dir = dirs.git_dirs().join(&proj_stub.dir);

        let repo = Repository::clone(url, &new_dir)?;

        let ref_name = inter.refs(&repo)?;
        proj_stub.ref_string = ref_name.to_string();
        let (obj, refe) = repo.revparse_ext(&ref_name)?;
        repo.checkout_tree(&obj, None)?;
        match refe {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(obj.id()),
        }?;

        let src_dir = dirs.src_dirs().join(&proj_stub.dir);
        std::fs::rename(&new_dir, &src_dir)?;
        let prj = inter.finish(proj_stub)?;

        project_store.add(prj.clone())?;
        self.script_runner(&prj, ScriptType::IScript)?;
        Ok(())
    }

    fn list<Q: MinorInteractions>(
        &self,
        pkg_name: Option<String>,
        inter: Q,
    ) -> Result<(), PMError> {
        let project_store = Self::Store::new()?;
        match pkg_name {
            Some(pkg) => {
                let project = project_store.get_ref(&pkg).ok_or(PMError::NonExisting)?;
                inter.list_one(&pkg, &project)?;
                Ok(())
            }
            None => {
                inter.list(&project_store)?;
                Ok(())
            }
        }
    }
    fn inter_edit<Q: MinorInteractions>(&self, package: &str, inter: Q) -> Result<(), PMError> {
        let project_store = Self::Store::new()?;
        if let Some(element) = project_store.get_clone(package) {
            let old_name = element.name.clone();
            let prj = inter.edit(element)?;
            self.edit(&old_name, prj)?;
        }
        Ok(())
    }
    fn inter_update<Q: MinorInteractions>(
        &self,
        pkg_name: Option<String>,
        force: bool,
        inter: Q,
    ) -> Result<(), PMError> {
        let project_store = Self::Store::new()?;
        match pkg_name {
            Some(package) => {
                project_store
                    .get_ref(&package)
                    .ok_or(PMError::NonExisting)?;
                self.update(&package)
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
