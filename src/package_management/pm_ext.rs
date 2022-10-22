use crate::{
    dirutils::PMDirs,
    package_management::{CommonError, PackageManagementCore, ScriptType},
    projects::{Project, ProjectStore, UpdatePolicy},
};

pub trait PackageManagementExt: PackageManagementCore {
    fn reinstall(&self, prj_name: &str) -> Result<(), Self::ErrorC> {
        let prj = Self::Store::new()?
            .get_clone(prj_name)
            .ok_or(CommonError::NonExisting)?;
        self.uninstall(prj_name)?;
        self.install(&prj)?;
        Ok(())
    }

    fn rebuild(&self, prj_name: &str) -> Result<(), Self::ErrorC> {
        let prj = Self::Store::new()?
            .get_clone(prj_name)
            .ok_or(CommonError::NonExisting)?;
        self.script_runner(&prj, ScriptType::IScript)?;
        Ok(())
    }

    fn bootstrap(&self) -> Result<(), Self::ErrorC> {
        let dirs = Self::Dirs::new();
        std::fs::create_dir_all(dirs.projects_db()).unwrap();
        std::fs::create_dir_all(dirs.suggestions_db()).unwrap();
        std::fs::create_dir_all(dirs.src_dirs()).unwrap();
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
