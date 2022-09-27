use crate::{package_management::*, projects::ProjectTable};
use pm_error::*;
use rayon::prelude::*;
use subprocess::Exec;

impl PackageManagementExt for PackageManager {
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
                        if !project_table.check_if_used_dir(
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
                        if !project_table.check_if_used_dir(
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
    fn reinstall(pkg_name: &str) -> Result<(), Self::Error> {
        let prj = ProjectTable::load()?
            .table
            .get_element(pkg_name)
            .ok_or(ReinstallError::NonExistant)?
            .info
            .clone();
        Self::uninstall(pkg_name)?;
        Self::install(pkg_name, &prj)?;
        Ok(())
    }
    fn rebuild(pkg_name: &str) -> Result<(), Self::Error> {
        let prj = ProjectTable::load()?
            .table
            .get_element(pkg_name)
            .ok_or(RebuildError::NonExistant)?
            .info
            .clone();
        let src_dir = dirutils::src_dirs().join(&prj.dir);
        let i_script = prj.install_script.join("&&");
        std::env::set_current_dir(&src_dir).map_err(CommonError::Path)?;
        if !Exec::shell(i_script).join()?.success() {
            return Err(RebuildError::Process.into());
        }
        Ok(())
    }
    fn bootstrap() -> Result<(), Self::Error> {
        use crate::projects::UpdatePolicy;
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
        Self::install("amisgitpm", &prj)
    }

    fn rename(old_package_name: &str, new_package_name: &str) -> Result<(), Self::Error> {
        ProjectTable::load()?
            .table
            .rename(old_package_name, new_package_name)?;
        Ok(())
    }
}
