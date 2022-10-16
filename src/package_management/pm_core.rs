use crate::{
    dirutils::PMDirs,
    package_management::{CommonError, PackageManagementBase, ScriptType},
    projects::{Project, ProjectStore},
};
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;

/// A trait whose Defaults are Sane, but bad.
pub trait PackageManagementCore: PackageManagementBase {
    fn install(&self, prj: &Project) -> Result<(), Self::Error> {
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
    fn uninstall(&self, prj_name: &str) -> Result<(), Self::Error> {
        let dirs = Self::Dirs::new();
        let mut project_store = Self::Store::new()?;
        let project = project_store
            .get_ref(prj_name)
            .ok_or(CommonError::NonExisting)?;
        self.script_runner(&project, ScriptType::UnIScript)?;
        let src_dir = dirs.src_dirs().join(&project.dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = dirs.old_dirs().join(&project.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        project_store.remove(prj_name)?;
        Ok(())
    }
    fn update(&self, prj_name: &str) -> Result<(), Self::Error> {
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
            repo.find_remote(remotes.get(0).unwrap_or("origin".into()))?
                .fetch(&[&prj.ref_string], None, None)?;
        }
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date(){
            return Ok(()); // early return
        }else if analysis.0.is_fast_forward(){
            let mut reference = repo.find_reference(&prj.dir)?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(&prj.ref_string)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            Err(CommonError::ImposibleUpdate(git_dir.clone(), prj.name.clone()))?;
        }
        self.build_rm(&prj, &git_dir)?;
        Ok(())
    }
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

    fn restore(&self, prj_name: &str) -> Result<(), Self::Error> {
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

    fn edit(&self, prj_name: &str, prj: Project) -> Result<(), Self::Error> {
        Self::Store::new()?.edit(prj_name, prj)?;
        Ok(())
    }

    fn cleanup(&self) -> Result<(), Self::Error> {
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
    use crate::package_management::{
        CommonError, PMError, PackageManagementCore, PackageManagerDefault,
    };
    use crate::projects::{Project, UpdatePolicy};
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
                pm.cleanup().unwrap();
                true
            } else {
                false
            }
        );
        pm.uninstall(&prj.name).unwrap();
    }
}
