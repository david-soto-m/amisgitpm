use crate::{
    dirutils::PMDirs,
    package_management::PMError,
    projects::{Project, ProjectStore},
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
    type Store: ProjectStore;
    type Dirs: PMDirs;
    fn install(&self, prj: &Project) -> Result<(), PMError> {
        let dirs = Self::Dirs::new();
        // Check there is directory is really new
        let mut project_store = Self::Store::new()?;
        if project_store.check_unique(&prj.name, &prj.dir) {
            return Err(PMError::AlreadyExisting);
        }
        // Create and clone the repo
        let new_dir = dirs.git_dirs().join(&prj.dir);
        let repo = Repository::clone(&prj.url, &new_dir)?;
        let (obj, refe) = repo.revparse_ext(&prj.ref_string)?;
        repo.checkout_tree(&obj, None)?;
        match refe {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(obj.id()),
        }?;
        // move everything to the src directory
        let src_dir = dirs.src_dirs().join(&prj.dir);
        std::fs::rename(&new_dir, &src_dir)?;
        // Push here, rebuild will work should the build fail
        project_store.add(prj.clone())?;
        self.script_runner(prj, ScriptType::IScript)?;
        Ok(())
    }

    fn uninstall(&self, pkg_name: &str) -> Result<(), PMError> {
        let dirs = Self::Dirs::new();
        let mut project_store = Self::Store::new()?;
        let project = project_store
            .get_ref(pkg_name)
            .ok_or(PMError::NonExisting)?;
        self.script_runner(&project, ScriptType::UnIScript)?;
        let src_dir = dirs.src_dirs().join(&project.dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = dirs.old_dirs().join(&project.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        project_store.remove(pkg_name)?;
        Ok(())
    }

    fn update(&self, prj: &str) -> Result<(), PMError> {
        todo!()
    }

    //     fn update(&self, pkg_name: &str) -> Result<(), PMError> {
    //         let src_dir = dirutils::src_dirs().join(&project.info.dir);
    //         let repo = Repository::open()?
    //
    //     }
    // fn fast_forward(&self, path: &Path) -> Result<(), Error> {
    //     let repo = Repository::open(path)?;
    //
    //     repo.find_remote("origin")?
    //         .fetch(&[self.branch], None, None)?;
    //
    //     let fetch_head = repo.find_reference("FETCH_HEAD")?;
    //     let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    //     let analysis = repo.merge_analysis(&[&fetch_commit])?;
    //     if analysis.0.is_up_to_date() {
    //         Ok(())
    //     } else if analysis.0.is_fast_forward() {
    //         let refname = format!("refs/heads/{}", self.branch);
    //         let mut reference = repo.find_reference(&refname)?;
    //         reference.set_target(fetch_commit.id(), "Fast-Forward")?;
    //         repo.set_head(&refname)?;
    //         repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
    //     } else {
    //         Err(Error::from_str("Fast-forward only!"))
    //     }
    // }

    fn restore(&self, pkg_name: &str) -> Result<(), PMError> {
        let dirs = Self::Dirs::new();
        let project = Self::Store::new()?
            .get_clone(pkg_name)
            .ok_or(PMError::NonExisting)?;
        let old_dir = dirs.old_dirs().join(&project.dir);
        let src_dir = dirs.src_dirs().join(&project.dir);
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
        let dirs = Self::Dirs::new();
        let src_dir = dirs.src_dirs().join(&prj.dir);
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
        Self::Store::new()?.edit(pkg_name, prj)?;
        Ok(())
    }

    fn cleanup(&self) -> Result<(), PMError> {
        let dirs = Self::Dirs::new();
        let project_store = Self::Store::new()?;
        let new_dir = dirs.git_dirs();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir)?;
        }
        let src_dir = dirs.src_dirs();
        if src_dir.exists() {
            std::fs::read_dir(src_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if !project_store.check_dir(entry.file_name().to_str().ok_or(PMError::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), PMError>(())
            })?;
        }
        let old_dir = dirs.old_dirs();
        if old_dir.exists() {
            std::fs::read_dir(&old_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if !project_store.check_dir(entry.file_name().to_str().ok_or(PMError::Os2str)?)
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
        assert!(directories::BaseDirs::new()
            .unwrap()
            .home_dir()
            .join(".local/bin/rust-hello-world")
            .exists());
        assert!(if let Err(PMError::AlreadyExisting) = pm.install(&prj) {
            true
        } else {
            false
        });
        pm.uninstall(&prj.name).unwrap();
    }
}