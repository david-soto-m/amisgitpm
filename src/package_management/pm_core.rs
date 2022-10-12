use crate::{
    dirutils,
    package_management::pm_error::PMError,
    projects::{Project, ProjectStore, ProjectTable},
};
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;
use rayon::prelude::*;
use subprocess::Exec;

#[derive(Debug)]
pub enum ScriptType {
    IScript,
    UnIScript,
}

pub trait PackageManagementCore {
    type Store: ProjectStore;
    fn install(&self, prj: &Project) -> Result<(), PMError> {
        // Check there is directory is really new
        let mut project_store = Self::Store::load()?;
        if project_store.check_unique(&prj.name, &prj.dir) {
            return Err(PMError::AlreadyExisting);
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
        std::fs::rename(&new_dir, &src_dir)?;
        // Push here, rebuild will work should the build fail
        project_store.add(prj.clone())?;
        self.script_runner(prj, ScriptType::IScript)?;
        Ok(())
    }

    fn uninstall(&self, pkg_name: &str) -> Result<(), PMError> {
        let mut project_store = ProjectTable::load()?;
        let project = project_store
            .table
            .get_element(pkg_name)
            .ok_or(PMError::NonExisting)?;
        self.script_runner(&project.info, ScriptType::UnIScript)?;
        let src_dir = dirutils::src_dirs().join(&project.info.dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = dirutils::old_src_dirs().join(&project.info.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        project_store.remove(pkg_name)?;
        Ok(())
    }

    fn update(&self, pkg_name: &str) -> Result<(), PMError> {
        todo!()
    }

    fn restore(&self, pkg_name: &str) -> Result<(), PMError> {
        let project = ProjectTable::load()?
            .table
            .get_element(pkg_name)
            .ok_or(PMError::NonExisting)?
            .info
            .clone();
        let old_dir = dirutils::old_src_dirs().join(&project.dir);
        let src_dir = dirutils::src_dirs().join(&project.dir);
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        std::fs::remove_dir_all(&src_dir)?;
        dir::copy(&old_dir, &src_dir, &opts)?;
        self.script_runner(&project, ScriptType::IScript)?;
        Ok(())
    }

    fn script_runner(&self, prj: &Project, scr_run: ScriptType) -> Result<(), PMError> {
        let src_dir = dirutils::src_dirs().join(&prj.dir);
        let script = match scr_run {
            ScriptType::IScript => prj.install_script.join("&&"),
            ScriptType::UnIScript => prj.uninstall_script.join("&&"),
        };
        std::env::set_current_dir(&src_dir)?;
        if !Exec::shell(script).join()?.success() {
            Err(PMError::Exec(prj.name.to_string(), scr_run))
        } else {
            Ok(())
        }
    }

    fn edit(&self, pkg_name: &str, prj: Project) -> Result<(), PMError> {
        let mut project_store = ProjectTable::load()?;
        if let Some(element) = project_store.table.get_mut_element(pkg_name) {
            element.info = prj;
        }
        Ok(())
    }

    fn cleanup(&self) -> Result<(), PMError> {
        let project_store = ProjectTable::load()?;
        let new_dir = dirutils::new_src_dirs();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir)?;
        }
        let src_dir = dirutils::src_dirs();
        if src_dir.exists() {
            std::fs::read_dir(src_dir)?.par_bridge().try_for_each(|e| {
                if let Ok(entry) = e {
                    if !project_store.check_dir(entry.file_name().to_str().ok_or(PMError::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), PMError>(())
            })?;
        }
        let old_dir = dirutils::old_src_dirs();
        if old_dir.exists() {
            std::fs::read_dir(&old_dir)?
                .par_bridge()
                .try_for_each(|e| {
                    if let Ok(entry) = e {
                        if !project_store
                            .check_dir(entry.file_name().to_str().ok_or(PMError::Os2str)?)
                        {
                            std::fs::remove_dir_all(entry.path())?;
                        }
                    }
                    Ok::<(), PMError>(())
                })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::package_management::{PMError, PackageManagementCore, PackageManager};
    use crate::projects::{Project, UpdatePolicy};
    #[test]
    fn install_uninstall_project() {
        let pm = PackageManager {};
        let prj = Project {
            name: "Hello-crate".into(),
            dir: "Hello-crate".into(),
            url: "https://github.com/zwang20/rust-hello-world.git".into(),
            ref_string: "refs/heads/master".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local".into()],
            uninstall_script: vec!["cargo uninstall --root ~/.local".into()],
        };
        pm.install(&prj).unwrap();
        assert!(directories::BaseDirs::new().unwrap().home_dir().join(".local/bin/rust-hello-world").exists());
        assert!(
            if let Err(PMError::AlreadyExisting) = pm.install(&prj) {
                true
            } else {
                false
            }
        );
        pm.uninstall(&prj.name).unwrap();
    }
}
