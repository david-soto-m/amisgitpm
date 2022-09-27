use crate::{
    dirutils,
    package_management::*,
    projects::{Project, ProjectTable},
};
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;
use pm_error::*;
use subprocess::Exec;

impl PackageManagementCore for PackageManager {
    type Error = PMError;
    fn install(pkg_name: &str, prj: &Project) -> Result<(), Self::Error> {
        // Check there is directory is really new
        let mut project_table = ProjectTable::load()?;
        if project_table.check_if_used_name_dir(pkg_name, &prj.dir) {
            return Err(InstallError::AlreadyExisting.into());
        }
        // Create and clone the repo
        let new_dir = dirutils::new_src_dirs().join(&prj.dir);
        let repo = Repository::clone(&prj.url, &new_dir)?;
        let (obj, refe) = repo.revparse_ext(&prj.ref_string)?;
        repo.checkout_tree(&obj, None)?;
        match refe {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(obj.id()),
        }?;
        // move everything to the src directory
        let src_dir = dirutils::src_dirs().join(&prj.dir);
        std::fs::rename(&new_dir, &src_dir).map_err(InstallError::Move)?;
        // Push here, rebuild will work should the build fail
        project_table.table.push(pkg_name, prj.clone())?;
        // Run the build script
        let i_script = prj.install_script.join("&&");
        std::env::set_current_dir(&src_dir).map_err(CommonError::Path)?;
        if !Exec::shell(i_script).join()?.success() {
            Err(InstallError::Process.into())
        } else {
            Ok(())
        }
    }

    fn uninstall(pkg_name: &str) -> Result<(), Self::Error> {
        let mut project_table = ProjectTable::load()?;
        let project = project_table
            .table
            .get_element(pkg_name)
            .ok_or(UninstallError::NonExistant)?;
        let src_dir = dirutils::src_dirs().join(&project.info.dir);
        std::env::set_current_dir(&src_dir).map_err(CommonError::Path)?;
        let rm_script = project.info.uninstall_script.join("&&");
        if !Exec::shell(rm_script).join()?.success() {
            return Err(UninstallError::Process.into());
        }
        std::fs::remove_dir_all(src_dir).map_err(UninstallError::Remove)?;
        let old_dir = dirutils::old_src_dirs().join(&project.info.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir).map_err(UninstallError::Remove)?;
        }
        project_table
            .table
            .pop(pkg_name)
            .map_err(|e| CommonError::Table(e).into())
    }

    fn update(pkg_name: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn restore(pkg_name: &str) -> Result<(), Self::Error> {
        let project_table = ProjectTable::load()?;
        let project = &project_table
            .table
            .get_element(pkg_name)
            .ok_or(RestoreError::NonExistant)?
            .info;
        let old_dir = dirutils::old_src_dirs().join(&project.dir);
        let new_dir = dirutils::new_src_dirs().join(&project.dir);
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        dir::copy(&old_dir, &new_dir, &opts).map_err(InstallError::Copy)?;
        std::env::set_current_dir(&new_dir).map_err(CommonError::Path)?;
        let i_script = project.install_script.join("&&");
        if !Exec::shell(i_script).join()?.success() {
            return Err(InstallError::Process.into());
        }
        let src_dir = dirutils::src_dirs().join(&project.dir);
        std::fs::rename(&src_dir, &old_dir).map_err(InstallError::Move)?;
        std::fs::rename(&new_dir, &src_dir).map_err(InstallError::Move)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::package_management::{
        pm_error::{InstallError, PMError},
        PackageManagementCore, PackageManager,
    };
    use crate::projects::{Project, UpdatePolicy};
    #[test]
    fn install_uninstall_project() {
        let prj = Project {
            dir: "Hello-crate".into(),
            url: "https://github.com/zwang20/rust-hello-world.git".into(),
            ref_string: "refs/heads/master".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local/".into()],
            uninstall_script: vec!["cargo uninstall rust-hello-world --root ~/.local/".into()],
        };
        PackageManager::install("Hello", &prj).unwrap();
        assert!(
            if let Err(PMError::Install(InstallError::AlreadyExisting)) =
                PackageManager::install("Hello", &prj)
            {
                true
            } else {
                false
            }
        );
        PackageManager::uninstall("Hello").unwrap();
    }
}
