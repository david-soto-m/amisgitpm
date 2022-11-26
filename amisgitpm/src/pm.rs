use crate::{Interactions, PMDirs, Project, ProjectStore, UpdatePolicy};
use git2::Repository;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ScriptType {
    IScript,
    UnIScript,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum CommonPMErrors {
    AlreadyExisting,
    NonExisting,
    Os2str,
    BadRef,
    ImposibleUpdate(PathBuf, String),
}
impl std::error::Error for CommonPMErrors {}
impl std::fmt::Display for CommonPMErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExisting => {
                write!(f, "A project with that name or directory already exists")
            }
            Self::NonExisting => write!(
                f,
                "That project that doesn't exist!
To list all projects use `amisgitpm list`"
            ),
            Self::Os2str => write!(f, "Couldn't convert from &Osstr to utf-8 &str"),
            Self::BadRef => write!(f, "Couldn't find a reference to a non detached HEAD"),
            Self::ImposibleUpdate(path, name) => write!(
                f,
                "Update couldn't be solved by a fast forward.
Solve the git problems in {path:?} and then run `amisgitpm rebuild {name} --from-git"
            ),
        }
    }
}

pub trait PMOperations
where
    Self: Sized,
{
    type Dirs: PMDirs;
    type Store: ProjectStore;
    type Error: std::error::Error + From<std::io::Error> + From<CommonPMErrors> + From<git2::Error>;
    fn new() -> Result<Self, Self::Error>;
    fn map_store_error(err: <Self::Store as ProjectStore>::Error) -> Self::Error;
    fn map_dir_error(err: <Self::Dirs as PMDirs>::Error) -> Self::Error;
    fn get_store(&self) -> &Self::Store;
    fn get_mut_store(&mut self) -> &mut Self::Store;
    fn get_dir(&self) -> &Self::Dirs;
    fn copy_directory<T: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: T,
        to: Q,
    ) -> Result<(), Self::Error>;
    fn download(&self, prj: &Project) -> Result<(Repository, PathBuf), Self::Error> {
        let git_dir = self.get_dir().git().join(&prj.dir);
        let repo = Repository::clone(&prj.url, &git_dir)?;
        Ok((repo, git_dir))
    }

    fn switch_branch(&self, prj: &Project, repo: &Repository) -> Result<(), Self::Error> {
        let (obj, refe) = repo.revparse_ext(&prj.ref_string)?;
        repo.checkout_tree(&obj, None)?;
        if let Some(gref) = refe {
            repo.set_head(gref.name().unwrap())?;
        } else {
            Err(CommonPMErrors::BadRef)?;
        }
        Ok(())
    }
    fn build_rm(&self, prj: &Project, path: &Path) -> Result<(), Self::Error> {
        let src_dir = self.get_dir().src().join(&prj.dir);
        if src_dir.exists() {
            std::fs::remove_dir_all(&src_dir)?;
        }
        self.copy_directory(path, &src_dir)?;
        std::fs::remove_dir_all(path)?;
        self.script_runner(prj, ScriptType::IScript)?;
        Ok(())
    }
    fn update_repo(&self, prj: &Project, repo: &Repository) -> Result<(), Self::Error> {
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
            Err(CommonPMErrors::ImposibleUpdate(
                self.get_dir().git().join(&prj.dir),
                prj.name.clone(),
            ))?;
        }
        Ok(())
    }
    fn script_runner(&self, prj: &Project, scr_run: ScriptType) -> Result<(), Self::Error>;
}

pub trait PMBasics: PMOperations {
    fn install(&mut self, prj: Project) -> Result<(), Self::Error> {
        if !self.get_store().check_unique(&prj.name, &prj.dir) {
            Err(CommonPMErrors::AlreadyExisting)?;
        }
        let (repo, git_dir) = self.download(&prj)?;
        self.switch_branch(&prj, &repo)?;
        self.get_mut_store()
            .add(prj.clone())
            .map_err(Self::map_store_error)?;
        self.build_rm(&prj, &git_dir)?;
        Ok(())
    }
    fn uninstall(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        let project = self
            .get_store()
            .get_ref(prj_name)
            .ok_or(CommonPMErrors::NonExisting)?;
        self.script_runner(project, ScriptType::UnIScript)?;
        let src_dir = self.get_dir().src().join(&project.dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = self.get_dir().old().join(&project.dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        self.get_mut_store()
            .remove(prj_name)
            .map_err(Self::map_store_error)?;
        Ok(())
    }
    fn update(&self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self
            .get_store()
            .get_ref(prj_name)
            .ok_or(CommonPMErrors::NonExisting)?;
        let git_dir = self.get_dir().git().join(&prj.dir);
        let old_dir = self.get_dir().old().join(&prj.dir);
        let src_dir = self.get_dir().src().join(&prj.dir);
        self.copy_directory(&src_dir, &old_dir)?;
        self.copy_directory(&src_dir, &git_dir)?;
        let repo = Repository::open(&git_dir)?;
        self.switch_branch(prj, &repo)?;
        self.update_repo(prj, &repo)?;
        self.build_rm(prj, &git_dir)?;
        Ok(())
    }
    fn restore(&self, prj_name: &str) -> Result<(), Self::Error> {
        let project = self
            .get_store()
            .get_ref(prj_name)
            .ok_or(CommonPMErrors::NonExisting)?;
        let old_dir = self.get_dir().old().join(&project.dir);
        let src_dir = self.get_dir().src().join(&project.dir);
        std::fs::remove_dir_all(&src_dir)?;
        self.copy_directory(&old_dir, &src_dir)?;
        self.script_runner(project, ScriptType::IScript)?;
        Ok(())
    }
    fn cleanup(&self) -> Result<(), Self::Error> {
        let new_dir = self.get_dir().git();
        if new_dir.exists() {
            std::fs::remove_dir_all(new_dir)?;
        }
        let src_dir = self.get_dir().src();
        if src_dir.exists() {
            std::fs::read_dir(src_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if self
                        .get_store()
                        .check_dir_free(entry.file_name().to_str().ok_or(CommonPMErrors::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), Self::Error>(())
            })?;
        }
        let old_dir = self.get_dir().old();
        if old_dir.exists() {
            std::fs::read_dir(&old_dir)?.try_for_each(|e| {
                if let Ok(entry) = e {
                    if !self
                        .get_store()
                        .check_dir_free(entry.file_name().to_str().ok_or(CommonPMErrors::Os2str)?)
                    {
                        std::fs::remove_dir_all(entry.path())?;
                    }
                }
                Ok::<(), Self::Error>(())
            })?;
        }
        Ok(())
    }
    fn edit(&mut self, prj_name: &str, prj: Project) -> Result<(), Self::Error> {
        self.get_mut_store()
            .edit(prj_name, prj)
            .map_err(Self::map_store_error)?;
        Ok(())
    }
}

pub trait PMExtended: PMBasics {
    fn reinstall(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self
            .get_store()
            .get_clone(prj_name)
            .ok_or(CommonPMErrors::NonExisting)?;
        self.uninstall(prj_name)?;
        self.install(prj)?;
        Ok(())
    }
    fn rebuild(&self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self
            .get_store()
            .get_ref(prj_name)
            .ok_or(CommonPMErrors::NonExisting)?;
        self.script_runner(prj, ScriptType::IScript)?;
        Ok(())
    }
    fn bootstrap(&mut self) -> Result<(), Self::Error>;
}
pub trait PMInteractive: PMBasics {
    type Interact: Interactions;
    fn map_inter_error(err: <Self::Interact as Interactions>::Error) -> Self::Error;
    fn inter_install(&mut self, url: &str) -> Result<(), Self::Error> {
        let url = if url.ends_with('/') {
            let (a, _) = url.rsplit_once('/').unwrap();
            a
        } else {
            url
        };
        let inter = Self::Interact::new().map_err(Self::map_inter_error)?;
        let sugg = url
            .split('/')
            .last()
            .map_or("temp".into(), |potential_dir| {
                potential_dir
                    .to_string()
                    .rsplit_once('.')
                    .map_or(potential_dir.to_string(), |(dir, _)| dir.to_string())
            });
        let mut proj_stub = Project {
            url: url.to_string(),
            dir: sugg,
            ..Default::default()
        };
        let (repo, git_dir) = self.download(&proj_stub)?;
        let ref_name = inter.refs(&repo).map_err(Self::map_inter_error)?;
        proj_stub.ref_string = ref_name;
        self.switch_branch(&proj_stub, &repo)?;
        let project = inter
            .create_project(&proj_stub, self.get_store(), &git_dir)
            .map_err(Self::map_inter_error)?;
        self.get_mut_store()
            .add(project.clone())
            .map_err(Self::map_store_error)?;
        self.build_rm(&project, &git_dir)?;
        Ok(())
    }
    fn list(&self, prj_names: Vec<String>) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::map_inter_error)?;
        if prj_names.is_empty() {
            inter
                .list(self.get_store())
                .map_err(Self::map_inter_error)?;
        } else {
            prj_names.into_iter().try_for_each(|prj_name| {
                let project = self
                    .get_store()
                    .get_ref(&prj_name)
                    .ok_or(CommonPMErrors::NonExisting)?;
                inter.list_one(project).map_err(Self::map_inter_error)?;
                Ok::<_, Self::Error>(())
            })?;
        }
        Ok(())
    }
    fn inter_edit(&mut self, package: &str) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::map_inter_error)?;
        if let Some(element) = self.get_store().get_clone(package) {
            let old_name = element.name.clone();
            let prj = inter.edit(element).map_err(Self::map_inter_error)?;
            self.edit(&old_name, prj)?;
        }
        Ok(())
    }
    fn inter_update(&self, prj_name: Option<String>, force: bool) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::map_inter_error)?;
        if let Some(package) = prj_name {
            self.get_store()
                .get_ref(&package)
                .ok_or(CommonPMErrors::NonExisting)?;
            self.update(&package)?;
            Ok(())
        } else {
            self.get_store()
                .iter()
                .filter(|e| match e.update_policy {
                    UpdatePolicy::Always => true,
                    UpdatePolicy::Ask => {
                        if force {
                            true
                        } else {
                            inter.update_confirm(&e.name).unwrap_or_default()
                        }
                    }
                    UpdatePolicy::Never => false,
                })
                .try_for_each(|e| self.update(&e.name))?;
            Ok(())
        }
    }
}
