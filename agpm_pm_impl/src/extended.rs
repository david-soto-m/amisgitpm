use crate::ProjectManager;
use agpm_abstract::*;

impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMExtended for ProjectManager<D, PS, I> {
    fn reinstall(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self
            .store
            .get_clone(prj_name)
            .ok_or(Self::Error::NonExisting)?;
        self.uninstall(prj_name)?;
        self.install(&prj)?;
        Ok(())
    }

    fn rebuild(&self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self
            .store
            .get_clone(prj_name)
            .ok_or(Self::Error::NonExisting)?;
        self.script_runner(&prj, ScriptType::IScript)?;
        Ok(())
    }

    fn bootstrap(&mut self) -> Result<(), Self::Error> {
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
