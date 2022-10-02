use crate::{
    build_suggestions::BuildSuggester,
    dirutils,
    interaction::{InstallInteractions, MinorInteractions},
    package_management::{pm_error::*, PackageManagementCore, ScriptType},
    projects::{ProjectTable, UpdatePolicy},
};
use git2::Repository;

pub trait PackageManagementInteractive: PackageManagementCore {
    fn inter_install<T, Q>(&self, url: &str, inter: T) -> Result<(), InstallError>
    where
        T: InstallInteractions,
        Q: BuildSuggester,
    {
        let mut project_table = ProjectTable::load()?;
        let (pkg_name, mut proj_stub) = inter
            .initial(url, &project_table)
            .map_err(InstallError::Interaction)?;
        let new_dir = dirutils::new_src_dirs().join(&proj_stub.dir);

        let repo = Repository::clone(url, &new_dir)?;

        let ref_name = inter.refs(&repo).map_err(InstallError::Interaction)?;
        proj_stub.ref_string = ref_name.to_string();
        let (obj, refe) = repo.revparse_ext(&ref_name)?;
        repo.checkout_tree(&obj, None)?;
        match refe {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(obj.id()),
        }?;

        let src_dir = dirutils::src_dirs().join(&proj_stub.dir);
        std::fs::rename(&new_dir, &src_dir).map_err(InstallError::Move)?;
        let suggester = <Q as BuildSuggester>::new(&src_dir).map_err(InstallError::Suggestions)?;
        let prj = inter
            .finish(proj_stub, suggester)
            .map_err(InstallError::Interaction)?;

        project_table.table.push(&pkg_name, prj.clone())?;
        self.script_runner(&pkg_name, &prj, ScriptType::IScript)?;
        Ok(())
    }

    fn list<Q: MinorInteractions>(
        &self,
        pkg_name: Option<String>,
        inter: Q,
    ) -> Result<(), ListError> {
        match pkg_name {
            Some(pkg) => {
                let project_table = ProjectTable::load()?;
                let project = project_table
                    .table
                    .get_element(&pkg)
                    .ok_or(ListError::NonExistant)?;
                inter
                    .list_one(&pkg, &project.info)
                    .map_err(ListError::Interaction)?;
                Ok(())
            }
            None => {
                let project_table = ProjectTable::load()?;
                inter.list(&project_table).map_err(ListError::Interaction)?;
                Ok(())
            }
        }
    }
    fn edit<Q: MinorInteractions>(&self, package: &str, inter: Q) -> Result<(), EditError> {
        let mut project_table = ProjectTable::load()?;
        if let Some(element) = project_table.table.get_mut_element(package) {
            inter
                .edit(&mut element.info)
                .map_err(EditError::Interaction)?;
        }
        Ok(())
    }
    fn inter_update<Q: MinorInteractions>(
        &self,
        pkg_name: Option<String>,
        force: bool,
        inter: Q,
    ) -> Result<(), UpdateError> {
        let project_table = ProjectTable::load()?;
        match pkg_name {
            Some(package) => {
                project_table
                    .table
                    .get_element(&package)
                    .ok_or(UpdateError::NonExistant)?;
                self.update(&package)
            }
            None => {
                project_table
                    .table
                    .iter()
                    .filter(|(name, e)| match e.info.update_policy {
                        UpdatePolicy::Always => true,
                        UpdatePolicy::Ask => {
                            if !force {
                                inter.update_confirm(name).unwrap_or_default()
                            } else {
                                true
                            }
                        }
                        UpdatePolicy::Never => false,
                    })
                    .try_for_each(|(_, e)| self.update(&e.info.dir))?;
                Ok(())
            }
        }
    }
}
