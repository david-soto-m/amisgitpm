use crate::{
    build_suggestions::BuildSuggester,
    dirutils,
    interaction::{InstallInteractions, MinorInteractions},
    package_management::*,
    projects::{Project, ProjectTable},
};
use git2::Repository;
use rayon::prelude::*;
use subprocess::Exec;
use fs_extra::{self, dir::CopyOptions};
use std::path::Path;
pub struct PackageManager {}

impl PackageManager {
    pub fn bootstrap() -> Result<(), PMError> {
        use crate::projects::UpdatePolicy;
        std::fs::create_dir_all(dirutils::projects_db()).unwrap();
        std::fs::create_dir_all(dirutils::suggestions_db()).unwrap();
        std::fs::create_dir_all(dirutils::src_dirs()).unwrap();
        let prj = Project {
            name: "amisgitpm".into(),
            url: "https://github.com/david-soto-m/amisgitpm.git".into(),
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local/".into()],
            uninstall_script: vec!["cargo uninstall amisgitpm --root ~/.local/".into()],
        };
        PackageManager::install(prj)
    }
}

impl PackageManagement for PackageManager {
    type Error = PMError;
    fn interactive_install<T, Q>(url: &str, path: Option<String>) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions,
    {
        let mut project_table = ProjectTable::load()?;
        let mut proj_stub = <Q as InstallInteractions>::initial(url, &project_table)
            .map_err(|e| InstallError::Interact(e.to_string()))?;
        let new_dir = dirutils::new_src_dirs().join(&proj_stub.name);

        let repo = match path{
            Some(path) =>{
                let path = Path::new(&path);
                if path.is_absolute(){
                    let opts= CopyOptions { overwrite: true,..Default::default()};
                    fs_extra::dir::copy(path, &new_dir, &opts)?;
                    Repository::open(&new_dir)?
                } else {
                    let new_dir = dirutils::new_src_dirs().join(path);
                    Repository::open(&new_dir)?
                }
            }
            None=> Repository::clone(url, &new_dir)?
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
    fn install(prj: Project) -> Result<(), Self::Error> {
        let mut project_table = ProjectTable::load()?;
        if project_table.check_if_used_name(&prj.name) {
            return Err(InstallError::AlreadyExisting.into());
        }
        let new_dir = dirutils::new_src_dirs().join(&prj.name);
        let repo = Repository::clone(&prj.url, &new_dir)?;
        let (obj, refe) = repo.revparse_ext(&prj.ref_string)?;
        repo.checkout_tree(&obj, None)?;
        match refe {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(obj.id()),
        }?;
        let i_script = prj.install_script.join("&&");
        std::env::set_current_dir(&new_dir).map_err(CommonError::Path)?;
        if !Exec::shell(i_script).join()?.success() {
            return Err(InstallError::Process.into());
        }
        let name = prj.name.clone();
        let src_dir = dirutils::src_dirs().join(&prj.name);
        std::fs::rename(new_dir, src_dir).map_err(InstallError::Move)?;
        project_table
            .table
            .push(name, prj)
            .map_err(|e| CommonError::Table(e).into())
    }
    fn uninstall(project: &str) -> Result<(), Self::Error> {
        let mut project_table = ProjectTable::load()?;
        let prj = project_table.table.get_element(project).ok_or(UninstallError::NonExistant)?;
        let src_dir = dirutils::src_dirs().join(&project);
        std::env::set_current_dir(&src_dir).map_err(CommonError::Path)?;
        let rm_script = prj.info.uninstall_script.join("&&");
        if !Exec::shell(rm_script).join()?.success() {
            return Err(UninstallError::Process.into());
        }
        std::fs::remove_dir_all(src_dir).map_err(UninstallError::Remove)?;
        let old_dir = dirutils::old_src_dirs().join(&project);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir).map_err(UninstallError::Remove)?;
        }
        project_table
            .table
            .pop(project)
            .map_err(|e| CommonError::Table(e).into())
    }
    fn reinstall(package: &str) -> Result<(), Self::Error>{
        let prj = ProjectTable::load()?
            .table
            .get_element(package)
            .ok_or(ReinstallError::NonExistant)?
            .info
            .clone();
        Self::uninstall(&prj.name)?;
        Self::install(prj)?;
        Ok(())
    }
    fn rebuild(package: &str) -> Result<(), Self::Error> {
        let prj = ProjectTable::load()?
            .table
            .get_element(package)
            .ok_or(RebuildError::NonExistant)?
            .info.clone();
        let src_dir = dirutils::src_dirs().join(&prj.name);
        let i_script = prj.install_script.join("&&");
        std::env::set_current_dir(&src_dir).map_err(CommonError::Path)?;
        if !Exec::shell(i_script).join()?.success() {
            return Err(RebuildError::Process.into());
        }
        Ok(())

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
    fn cleanup() -> Result<(), Self::Error> {
        let project_table = ProjectTable::load()?;
        let new_dir = dirutils::new_src_dirs();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir).map_err(CleanupError::FileOp)?;
        }
        let src_dir = dirutils::src_dirs();
        if src_dir.exists() {
            std::fs::read_dir(src_dir)
                .map_err(CleanupError::FileOp)?
                .par_bridge()
                .try_for_each(|e| {
                    if let Ok(entry) = e {
                        if !project_table.check_if_used_name(
                            entry.file_name().to_str().ok_or(CleanupError::String)?,
                        ) {
                            std::fs::remove_dir_all(entry.path()).map_err(CleanupError::FileOp)?;
                        }
                    }
                    Ok::<(), CleanupError>(())
                })?;
        }
        let old_dir = dirutils::old_src_dirs();
        if old_dir.exists() {
            std::fs::remove_dir_all(&old_dir).map_err(CleanupError::FileOp)?;
            std::fs::read_dir(&old_dir)
                .map_err(CleanupError::FileOp)?
                .par_bridge()
                .try_for_each(|e| {
                    if let Ok(entry) = e {
                        if !project_table.check_if_used_name(
                            entry.file_name().to_str().ok_or(CleanupError::String)?,
                        ) {
                            std::fs::remove_dir_all(entry.path()).map_err(CleanupError::FileOp)?;
                        }
                    }
                    Ok::<(), CleanupError>(())
                })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::package_management::{InstallError, PMError, PackageManagement, PackageManager};
    use crate::projects::{Project, UpdatePolicy};
    #[test]
    fn install_uninstall_project() {
        let prj = Project {
            name: "Hello-crate".into(),
            url: "https://github.com/zwang20/rust-hello-world.git".into(),
            ref_string: "refs/heads/master".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local/".into()],
            uninstall_script: vec!["cargo uninstall rust-hello-world --root ~/.local/".into()],
        };
        PackageManager::install(prj.clone()).unwrap();
        assert!(
            if let Err(PMError::Install(InstallError::AlreadyExisting)) =
                PackageManager::install(prj)
            {
                true
            } else {
                false
            }
        );
        PackageManager::uninstall("Hello-crate").unwrap();
    }
}
