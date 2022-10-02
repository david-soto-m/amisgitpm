use crate::{
    dirutils,
    package_management::pm_error::*,
    projects::{Project, ProjectTable},
};
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;
use subprocess::Exec;

#[derive(Debug)]
pub enum ScriptType {
    IScript,
    UnIScript,
}

pub trait PackageManagementCore {
    fn install(&self, pkg_name: &str, prj: &Project) -> Result<(), InstallError> {
        // Check there is directory is really new
        let mut project_table = ProjectTable::load()?;
        if project_table.check_if_used_name_dir(pkg_name, &prj.dir) {
            return Err(InstallError::AlreadyExisting);
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
        self.script_runner(pkg_name, prj, ScriptType::IScript)?;
        Ok(())
    }

    fn uninstall(&self, pkg_name: &str) -> Result<(), UninstallError> {
        let mut project_table = ProjectTable::load()?;
        let project = project_table
            .table
            .get_element(pkg_name)
            .ok_or(UninstallError::NonExistant)?;
        self.script_runner(pkg_name, &project.info, ScriptType::IScript)?;
        let src_dir = dirutils::src_dirs().join(&project.info.dir);
        std::fs::remove_dir_all(src_dir).map_err(UninstallError::Remove)?;
        let old_dir = dirutils::old_src_dirs().join(&project.info.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir).map_err(UninstallError::Remove)?;
        }
        project_table.table.pop(pkg_name)?;
        Ok(())
    }

    fn update(&self, pkg_name: &str) -> Result<(), UpdateError> {
        todo!()
    }

    fn restore(&self, pkg_name: &str) -> Result<(), RestoreError> {
        let project = ProjectTable::load()?
            .table
            .get_element(pkg_name)
            .ok_or(RestoreError::NonExistant)?
            .info
            .clone();
        let old_dir = dirutils::old_src_dirs().join(&project.dir);
        let src_dir = dirutils::src_dirs().join(&project.dir);
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        std::fs::remove_dir_all(&src_dir).map_err(RestoreError::Remove)?;
        dir::copy(&old_dir, &src_dir, &opts).map_err(RestoreError::Copy)?;
        self.script_runner(pkg_name, &project, ScriptType::IScript)?;
        Ok(())
    }

    fn script_runner(
        &self,
        pkg_name: &str,
        prj: &Project,
        scr_run: ScriptType,
    ) -> Result<(), ScriptError> {
        let src_dir = dirutils::src_dirs().join(&prj.dir);
        let script = match scr_run {
            ScriptType::IScript => prj.install_script.join("&&"),
            ScriptType::UnIScript => prj.uninstall_script.join("&&"),
        };
        std::env::set_current_dir(&src_dir).map_err(ScriptError::Path)?;
        if !Exec::shell(script).join()?.success() {
            Err(ScriptError::Exec(pkg_name.to_string(), scr_run))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::package_management::{
        pm_core_err::InstallError, PackageManagementCore, PackageManager,
    };
    use crate::projects::{Project, UpdatePolicy};
    #[test]
    fn install_uninstall_project() {
        let pm = PackageManager {};
        let prj = Project {
            dir: "Hello-crate".into(),
            url: "https://github.com/zwang20/rust-hello-world.git".into(),
            ref_string: "refs/heads/master".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local/".into()],
            uninstall_script: vec!["cargo uninstall rust-hello-world --root ~/.local/".into()],
        };
        pm.install("Hello", &prj).unwrap();
        assert!(
            if let Err(InstallError::AlreadyExisting) = pm.install("Hello", &prj) {
                true
            } else {
                false
            }
        );
        pm.uninstall("Hello").unwrap();
    }
}
