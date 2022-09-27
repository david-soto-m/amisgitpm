use crate::{
    build_suggestions::BuildSuggester,
    dirutils,
    interaction::{InstallInteractions, MinorInteractions},
    package_management::*,
    projects::{ProjectTable, UpdatePolicy},
};
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;
use pm_error::*;
use std::path::Path;
use subprocess::Exec;

impl PackageManagementInteractive for PackageManager {
    fn inter_install<T, Q>(url: &str, path: Option<String>) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions,
    {
        let mut project_table = ProjectTable::load()?;
        let (pkg_name, mut proj_stub) = <Q as InstallInteractions>::initial(url, &project_table)
            .map_err(|e| InstallError::Interact(e.to_string()))?;
        let new_dir = dirutils::new_src_dirs().join(&proj_stub.dir);

        let repo = match path {
            Some(path) => {
                let path = Path::new(&path);
                let opts = CopyOptions {
                    overwrite: true,
                    copy_inside: true,
                    ..Default::default()
                };
                dir::copy(&path, &new_dir, &opts).map_err(InstallError::Copy)?;
                Repository::open(&path)?
            }
            None => Repository::clone(url, &new_dir)?,
        };

        let ref_name = <Q as InstallInteractions>::refs(&repo)
            .map_err(|e| InstallError::Interact(e.to_string()))?;
        proj_stub.ref_string = ref_name.to_string();
        let (obj, refe) = repo.revparse_ext(&ref_name)?;
        repo.checkout_tree(&obj, None)?;
        match refe {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(obj.id()),
        }?;

        let src_dir = dirutils::src_dirs().join(&proj_stub.dir);
        std::fs::rename(&new_dir, &src_dir).map_err(InstallError::Move)?;
        let a = <T as BuildSuggester>::new(&src_dir)
            .map_err(|e| InstallError::Suggestions(e.to_string()))?;
        let prj = <Q as InstallInteractions>::finish(proj_stub, a)
            .map_err(|e| InstallError::Interact(e.to_string()))?;

        project_table.table.push(pkg_name, prj.clone())?;

        let i_script = prj.install_script.join("&&");
        std::env::set_current_dir(&src_dir).map_err(CommonError::Path)?;
        if !Exec::shell(i_script).join()?.success() {
            return Err(InstallError::Process.into());
        } else {
            Ok(())
        }
    }

    fn list<Q: MinorInteractions>(pkg_name: Option<String>) -> Result<(), Self::Error> {
        match pkg_name {
            Some(pkg) => {
                let project_table = ProjectTable::load()?;
                let project = project_table
                    .table
                    .get_element(&pkg)
                    .ok_or(UninstallError::NonExistant)?;
                <Q as MinorInteractions>::list_one(&pkg, &project.info)
                    .map_err(|e| ListError::Interact(e.to_string()))?;
                Ok(())
            },
            None => {
                let project_table = ProjectTable::load()?;
                <Q as MinorInteractions>::list(&project_table)
                    .map_err(|e| ListError::Interact(e.to_string()))?;
                Ok(())
            }
        }
    }
    fn edit<Q: MinorInteractions>(package: &str) -> Result<(), Self::Error> {
        let mut project_table = ProjectTable::load()?;
        if let Some(element) = project_table.table.get_mut_element(package) {
            <Q as MinorInteractions>::edit(&mut element.info)
                .map_err(|e| EditError::Interact(e.to_string()))?;
        }
        Ok(())
    }
    fn inter_update<Q: UpdateInteractions>(
        pkg_name: Option<String>,
        force: bool,
    ) -> Result<(), Self::Error> {
        let project_table = ProjectTable::load()?;
        match pkg_name {
            Some(package) => {
                project_table
                    .table
                    .get_element(&package)
                    .ok_or(UpdateError::NonExistant)?;
                Self::update(&package)
            }
            None => {
                project_table
                    .table
                    .iter()
                    .filter(|(name, e)|{
                        match e.info.update_policy {
                            UpdatePolicy::Always => true,
                            UpdatePolicy::Ask => {
                                if !force {
                                    <Q as UpdateInteractions>::confirm(name).unwrap_or_default()
                                } else {
                                    true
                                }
                            }
                            UpdatePolicy::Never => false,
                        }
                    })
                    .try_for_each(|(_, e)| Self::update(&e.info.dir))?;
                Ok(())
            }
        }
    }
}
