use crate::{
    build_suggestions::BuildSuggester,
    dirutils,
    interaction::InstallInteractions,
    projects::{Project, ProjectTable},
};
use git2::Repository;
mod gitutil_error;
pub use gitutil_error::{GitUtilError, InstallError, UninstallError, CommonError};
use subprocess::Exec;

pub trait GitUtils {
    type Error: std::error::Error;
    fn interactive_install<T, Q>(url: &str) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions;
    fn install(prj: Project) -> Result<(), Self::Error>;
    fn uninstall(string: &str) -> Result<(), Self::Error>;
}

pub struct GitUtilImpl {}

impl GitUtils for GitUtilImpl {
    type Error = GitUtilError;
    fn interactive_install<T, Q>(url: &str) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions,
    {
        let mut project_table = ProjectTable::load()?;
        let mut proj_stub = <Q as InstallInteractions>::initial(url, &project_table)
            .map_err(|e| CommonError::Interact(e.to_string()))?;
        let new_dir = dirutils::new_src_dirs().join(&proj_stub.name);
        println!("Starting download, please, wait a bit");
        let repo = Repository::clone(url, &new_dir)?;
        let ref_name = <Q as InstallInteractions>::refs(&repo)
            .map_err(|e| CommonError::Interact(e.to_string()))?;
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
            .map_err(|e| CommonError::Interact(e.to_string()))?;
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
        if let Some(prj) = project_table.table.get_element(project) {
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
            project_table.table.pop(project)
            .map_err(|e| CommonError::Table(e).into())
        }else{
            Err(UninstallError::NonExistant.into())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn install_project() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
