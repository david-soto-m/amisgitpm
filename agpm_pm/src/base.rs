use crate::ProjectManager;
use agpm_abstract::*;
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;

impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMBasics for ProjectManager<D, PS, I> {
    type Store = PS;
    fn install(&mut self, prj: &Project) -> Result<(), Self::Error> {
        if !self.store.check_unique(&prj.name, &prj.dir) {
            Err(Self::Error::AlreadyExisting)?;
        }
        let (repo, git_dir) = self.download(prj)?;
        self.switch_branch(prj, &repo)?;
        self.store.add(prj.clone()).map_err(Self::Error::Store)?;
        self.build_rm(prj, &git_dir)?;
        Ok(())
    }
    fn uninstall(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        let project = self
            .store
            .get_ref(prj_name)
            .ok_or(Self::Error::NonExisting)?;
        self.script_runner(project, ScriptType::UnIScript)?;
        let src_dir = self.dirs.src_dirs().join(&project.dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = self.dirs.old_dirs().join(&project.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        self.store.remove(prj_name).map_err(Self::Error::Store)?;
        Ok(())
    }
    fn update(&self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self
            .store
            .get_ref(prj_name)
            .ok_or(Self::Error::NonExisting)?;
        let git_dir = self.dirs.git_dirs().join(&prj.dir);
        let old_dir = self.dirs.old_dirs().join(&prj.dir);
        let src_dir = self.dirs.src_dirs().join(&prj.dir);
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
            Err(Self::Error::ImposibleUpdate(
                git_dir.clone(),
                prj.name.clone(),
            ))?;
        }
        self.build_rm(&prj, &git_dir)?;
        Ok(())
    }

    fn restore(&self, prj_name: &str) -> Result<(), Self::Error> {
        let project = self
            .store
            .get_ref(prj_name)
            .ok_or(Self::Error::NonExisting)?;
        let old_dir = self.dirs.old_dirs().join(&project.dir);
        let src_dir = self.dirs.src_dirs().join(&project.dir);
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

    fn edit(&mut self, prj_name: &str, prj: Project) -> Result<(), Self::Error> {
        self.store.edit(prj_name, prj).map_err(Self::Error::Store)?;
        Ok(())
    }

    fn cleanup(&self) -> Result<(), Self::Error> {
        let new_dir = self.dirs.git_dirs();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir)?;
        }
        let src_dir = self.dirs.src_dirs();
        if src_dir.exists() {
            std::fs::read_dir(src_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if self
                        .store
                        .check_dir_free(entry.file_name().to_str().ok_or(Self::Error::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), Self::Error>(())
            })?;
        }
        let old_dir = self.dirs.old_dirs();
        if old_dir.exists() {
            std::fs::read_dir(&old_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if !self
                        .store
                        .check_dir_free(entry.file_name().to_str().ok_or(Self::Error::Os2str)?)
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
