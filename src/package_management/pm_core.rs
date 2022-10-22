use crate::{
    dirutils::PMDirs,
    package_management::{CommonError, PackageManagementBase, ScriptType},
    projects::{Project, ProjectStore},
};
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;

/// A trait whose Defaults are Sane, but bad.
pub trait PackageManagementCore: PackageManagementBase {
    type Store: ProjectStore;
    type ErrorC: std::error::Error
        + From<Self::Error>
        + From<CommonError>
        + From<<Self::Store as ProjectStore>::Error>
        + From<std::io::Error>
        + From<git2::Error>
        + From<fs_extra::error::Error>
        + From<subprocess::PopenError>;

    fn install(&self, prj: &Project) -> Result<(), Self::ErrorC> {
        let mut project_store = Self::Store::new()?;
        if !project_store.check_unique(&prj.name, &prj.dir) {
            Err(CommonError::AlreadyExisting)?;
        }
        let (repo, git_dir) = self.download(prj)?;
        self.switch_branch(prj, &repo)?;
        project_store.add(prj.clone())?;
        self.build_rm(prj, &git_dir)?;
        Ok(())
    }
    fn uninstall(&self, prj_name: &str) -> Result<(), Self::ErrorC> {
        let dirs = Self::Dirs::new();
        let mut project_store = Self::Store::new()?;
        let project = project_store
            .get_ref(prj_name)
            .ok_or(CommonError::NonExisting)?;
        self.script_runner(project, ScriptType::UnIScript)?;
        let src_dir = dirs.src_dirs().join(&project.dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = dirs.old_dirs().join(&project.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        project_store.remove(prj_name)?;
        Ok(())
    }
    fn update(&self, prj_name: &str) -> Result<(), Self::ErrorC> {
        let dirs = Self::Dirs::new();
        let prj = Self::Store::new()?
            .get_clone(prj_name)
            .ok_or(CommonError::NonExisting)?;
        let git_dir = dirs.git_dirs().join(&prj.dir);
        let old_dir = dirs.old_dirs().join(&prj.dir);
        let src_dir = dirs.src_dirs().join(&prj.dir);
        let opts = CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        };
        dir::copy(&src_dir, &old_dir, &opts)?;
        dir::copy(&src_dir, &git_dir, &opts)?;
        let repo = Repository::open(&git_dir)?;
        self.switch_branch(&prj, &repo)?;
        let remotes = repo.remotes()?;
        if !remotes.is_empty() {
            repo.find_remote(remotes.get(0).unwrap_or("origin"))?
                .fetch(&[&prj.ref_string], None, None)?;
        }
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date() {
            return Ok(()); // early return
        } else if analysis.0.is_fast_forward() {
            let mut reference = repo.find_reference(&prj.ref_string)?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(&prj.ref_string)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            Err(CommonError::ImposibleUpdate(
                git_dir.clone(),
                prj.name.clone(),
            ))?;
        }
        self.build_rm(&prj, &git_dir)?;
        Ok(())
    }

    fn restore(&self, prj_name: &str) -> Result<(), Self::ErrorC> {
        let dirs = Self::Dirs::new();
        let project = Self::Store::new()?
            .get_clone(prj_name)
            .ok_or(CommonError::NonExisting)?;
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

    fn edit(&self, prj_name: &str, prj: Project) -> Result<(), Self::ErrorC> {
        Self::Store::new()?.edit(prj_name, prj)?;
        Ok(())
    }

    fn cleanup(&self) -> Result<(), Self::ErrorC> {
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
                    if project_store
                        .check_dir_free(entry.file_name().to_str().ok_or(CommonError::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), Self::Error>(())
            })?;
        }
        let old_dir = dirs.old_dirs();
        if old_dir.exists() {
            std::fs::read_dir(&old_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if !project_store
                        .check_dir_free(entry.file_name().to_str().ok_or(CommonError::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), Self::Error>(())
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dirutils::PMDirs,
        package_management::{
            CommonError, PMError, PackageManagementBase, PackageManagementCore,
            PackageManagerDefault,
        },
        projects::{Project, UpdatePolicy},
    };
    use std::{fs::canonicalize, io::prelude::*, path::PathBuf};
    use subprocess::Exec;
    #[test]
    fn install_uninstall_project() {
        let pm = PackageManagerDefault {};
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
        assert!(
            if let Err(PMError::Commons(CommonError::AlreadyExisting)) = pm.install(&prj) {
                true
            } else {
                false
            }
        );
        pm.uninstall(&prj.name).unwrap();
    }
    #[test]
    fn updates() {
        let dir = canonicalize(PathBuf::from(".").join("tests/projects/git_upd")).unwrap();
        assert_eq!(
            Exec::shell("bash 0_start.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        let mut url: String = "file://".into();
        url.push_str(&dir.to_str().unwrap());
        let prj = Project {
            name: "git_upd".into(),
            dir: "git_upd".into(),
            url,
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec![],
            uninstall_script: vec![],
        };
        let pm = PackageManagerDefault::new().unwrap();
        let a = <PackageManagerDefault as PackageManagementBase>::Dirs::new();
        pm.install(&prj).unwrap();
        let mut epoch = String::new();
        std::fs::File::open(dir.join("dates.txt"))
            .unwrap()
            .read_to_string(&mut epoch)
            .unwrap();
        let epoch = epoch.trim().parse::<i64>().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(
            Exec::shell("bash 1_update.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        let mut epoch2 = String::new();
        std::fs::File::open(dir.join("dates.txt"))
            .unwrap()
            .read_to_string(&mut epoch2)
            .unwrap();
        let epoch2 = epoch2.trim().parse::<i64>().unwrap();
        assert!(epoch2 > epoch);
        pm.update("git_upd").unwrap();
        let mut epoch2 = String::new();
        std::fs::File::open(a.src_dirs().join("git_upd").join("dates.txt"))
            .unwrap()
            .read_to_string(&mut epoch2)
            .unwrap();
        let epoch2 = epoch2.trim().parse::<i64>().unwrap();
        assert!(epoch2 > epoch);
        assert_eq!(
            Exec::shell("bash 2_finish.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        pm.uninstall("git_upd").unwrap();
    }
}
