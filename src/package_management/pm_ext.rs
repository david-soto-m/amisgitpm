use crate::{
    dirutils,
    package_management::{pm_error::*, PackageManagementCore, ScriptType},
    projects::{Project, ProjectTable, UpdatePolicy},
};
use rayon::prelude::*;

pub trait PackageManagementExt: PackageManagementCore {
    fn cleanup(&self) -> Result<(), CleanupError> {
        let project_table = ProjectTable::load()?;
        let new_dir = dirutils::new_src_dirs();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir).map_err(CleanupError::Remove)?;
        }
        let src_dir = dirutils::src_dirs();
        if src_dir.exists() {
            std::fs::read_dir(src_dir)
                .map_err(CleanupError::Read)?
                .par_bridge()
                .try_for_each(|e| {
                    if let Ok(entry) = e {
                        if !project_table.check_if_used_dir(
                            entry.file_name().to_str().ok_or(CleanupError::Os2str)?,
                        ) {
                            std::fs::remove_dir_all(entry.path()).map_err(CleanupError::Remove)?;
                        }
                    }
                    Ok::<(), CleanupError>(())
                })?;
        }
        let old_dir = dirutils::old_src_dirs();
        if old_dir.exists() {
            std::fs::read_dir(&old_dir)
                .map_err(CleanupError::Read)?
                .par_bridge()
                .try_for_each(|e| {
                    if let Ok(entry) = e {
                        if !project_table.check_if_used_dir(
                            entry.file_name().to_str().ok_or(CleanupError::Os2str)?,
                        ) {
                            std::fs::remove_dir_all(entry.path()).map_err(CleanupError::Remove)?;
                        }
                    }
                    Ok::<(), CleanupError>(())
                })?;
        }
        Ok(())
    }

    fn reinstall(&self, pkg_name: &str) -> Result<(), ReinstallError> {
        let prj = ProjectTable::load()?
            .table
            .get_element(pkg_name)
            .ok_or(ReinstallError::NonExistant)?
            .info
            .clone();
        self.uninstall(pkg_name)?;
        self.install(pkg_name, &prj)?;
        Ok(())
    }

    fn rebuild(&self, pkg_name: &str) -> Result<(), RebuildError> {
        let prj = ProjectTable::load()?
            .table
            .get_element(pkg_name)
            .ok_or(RebuildError::NonExistant)?
            .info
            .clone();
        self.script_runner(pkg_name, &prj, ScriptType::IScript)?;
        Ok(())
    }
    fn bootstrap(&self) -> Result<(), InstallError> {
        std::fs::create_dir_all(dirutils::projects_db()).unwrap();
        std::fs::create_dir_all(dirutils::suggestions_db()).unwrap();
        std::fs::create_dir_all(dirutils::src_dirs()).unwrap();
        let prj = Project {
            dir: "amisgitpm".into(),
            url: "https://github.com/david-soto-m/amisgitpm.git".into(),
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local/".into()],
            uninstall_script: vec!["cargo uninstall amisgitpm --root ~/.local/".into()],
        };
        self.install("amisgitpm", &prj)
    }

    fn rename(&self, old_package_name: &str, new_package_name: &str) -> Result<(), RenameError> {
        ProjectTable::load()?
            .table
            .rename(old_package_name, new_package_name)?;
        Ok(())
    }
}
