use crate::ProjectManager;
use agpm_abstract::*;
use fs_extra::dir::{self, CopyOptions};
use git2::Repository;

impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMBasics for ProjectManager<D, PS, I> {
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
            .get_ref_from_store(prj_name)
            .ok_or(Self::Error::NonExisting)?;
        let old_dir = self.old_dir().join(&project.dir);
        let src_dir = self.src_dir().join(&project.dir);
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
}
