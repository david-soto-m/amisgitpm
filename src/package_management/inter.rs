use crate::{
    build_suggestions::BuildSuggester,
    dirutils,
    interaction::{InstallInteractions, MinorInteractions},
    package_management::*,
    projects::{ProjectTable, UpdatePolicy},
};
use git2::Repository;
use pm_error::*;
use rayon::prelude::*;
use std::path::Path;
use subprocess::Exec;

impl PackageManagementInteractive for PackageManager {
    fn inter_install<T, Q>(url: &str, path: Option<String>) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions,
    {
        let mut project_table = ProjectTable::load()?;
        let mut proj_stub = <Q as InstallInteractions>::initial(url, &project_table)
            .map_err(|e| InstallError::Interact(e.to_string()))?;
        let new_dir = dirutils::new_src_dirs().join(&proj_stub.name);

        let repo = match path {
            Some(path) => {
                let path = Path::new(&path);
                let new_dir = dirutils::new_src_dirs().join(path);
                Repository::open(&new_dir)?
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
        let name = proj_stub.name.to_owned();
        let a = <T as BuildSuggester>::new(&new_dir)
            .map_err(|e| InstallError::Suggestions(e.to_string()))?;
        let prj = <Q as InstallInteractions>::finish(proj_stub, a)
            .map_err(|e| InstallError::Interact(e.to_string()))?;
        let i_script = prj.install_script.join("&&");
        std::env::set_current_dir(&new_dir).map_err(CommonError::Path)?;
        if !Exec::shell(i_script).join()?.success() {
            return Err(InstallError::Process.into());
        }
        let src_dir = dirutils::src_dirs().join(&prj.name);
        std::fs::rename(new_dir, src_dir).map_err(InstallError::Move)?;
        project_table
            .table
            .push(&name, prj)
            .map_err(|e| CommonError::Table(e).into())
    }

    fn list<Q: MinorInteractions>() -> Result<(), Self::Error> {
        let project_table = ProjectTable::load()?;
        <Q as MinorInteractions>::list(&project_table)
            .map_err(|e| ListError::Interact(e.to_string()))?;
        Ok(())
    }
    fn edit<Q: MinorInteractions>(package: &str) -> Result<(), Self::Error> {
        let mut project_table = ProjectTable::load()?;
        if let Some(element) = project_table.table.get_mut_element(package) {
            <Q as MinorInteractions>::edit(&mut element.info)
                .map_err(|e| EditError::Interact(e.to_string()))?;
        }
        Ok(())
    }
    fn inter_update<Q: UpdateInteractions>(package: Option<String>) -> Result<(), Self::Error> {
        let project_table = ProjectTable::load()?;
        match package {
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
                    .get_info_iter()
                    .filter(|e| match e.update_policy {
                        UpdatePolicy::Always => true,
                        UpdatePolicy::Ask => <Q as UpdateInteractions>::confirm(&e.name).unwrap_or_default(),
                        UpdatePolicy::Never => false,
                    }).try_for_each(|e|{
                        Self::update(&e.name)
                    })?;
                Ok(())
            }
        }
    }
}
