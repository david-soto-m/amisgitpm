use crate::{
    dirutils,
    package_management::{PMError, PackageManagementCore, ScriptType},
    projects::{Project, ProjectStore, UpdatePolicy},
};

pub trait PackageManagementExt: PackageManagementCore {
    fn reinstall(&self, pkg_name: &str) -> Result<(), PMError> {
        let prj = Self::Store::load()?
            .get_clone(pkg_name)
            .ok_or(PMError::NonExisting)?;
        self.uninstall(pkg_name)?;
        self.install(&prj)?;
        Ok(())
    }

    fn rebuild(&self, pkg_name: &str) -> Result<(), PMError> {
        let prj = Self::Store::load()?
            .get_clone(pkg_name)
            .ok_or(PMError::NonExisting)?;
        self.script_runner(&prj, ScriptType::IScript)?;
        Ok(())
    }
    fn bootstrap(&self) -> Result<(), PMError> {
        std::fs::create_dir_all(dirutils::projects_db()).unwrap();
        std::fs::create_dir_all(dirutils::suggestions_db()).unwrap();
        std::fs::create_dir_all(dirutils::src_dirs()).unwrap();
        let prj = Project {
            name: "amisgitpm".into(),
            dir: "amisgitpm".into(),
            url: "https://github.com/david-soto-m/amisgitpm.git".into(),
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local/".into()],
            uninstall_script: vec!["cargo uninstall amisgitpm --root ~/.local/".into()],
        };
        self.install(&prj)
    }
}
