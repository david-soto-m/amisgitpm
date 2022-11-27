//! This crate defines the traits for a package manager.
//!
//! An amisgitpm compliant package manager must do six tasks
//!
//! 1. Install from an URL
//! 2. Know what has been installed
//! 3. Update installed projects
//! 4. Edit project setups
//! 5. Go back to the previous version
//! 6. Uninstall projects
//!
//! The `PMOperations` is a trait that makes easy implementing the other traits
//! easier or automatic. It does no "high level" operation, but is concerned with
//! the procedures to make the package manager work. Most of the methods that
//! are not provided are very straight forward to implement.
//!
//! The API facing traits is `PMProgramatic` It is completely provided for you. It
//! does the six tasks
//!
//! The "User" facing traits is `PMInteractive`.
//! All methods must be implemented, but will frequently just run methods from
//! `PMProgramatic` or reimplement some other using `PMOperations`
//!

use crate::{PMDirs, ProjectStore, ProjectT};
use git2::Repository;
use std::path::{Path, PathBuf};

/// An error class that's needed to provide methods
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum CommonPMErrors {
    /// Attempting to install an already existing project
    AlreadyExisting,
    /// Attempting to get a non existing project from the store
    NonExisting,
    /// Couldn't parse an OsStr as a utf-8 str
    Os2Str,
    /// Can't find a reference to detached head
    BadRef,
    /// Couldn't update with a fast forward
    ImposibleUpdate,
}
impl std::error::Error for CommonPMErrors {}
impl std::fmt::Display for CommonPMErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExisting => {
                write!(f, "A project with that name or directory already exists")
            }
            Self::NonExisting => write!(f,"That project that doesn't exist!"),
            Self::Os2Str => write!(f, "Couldn't convert from &Osstr to utf-8 &str"),
            Self::BadRef => write!(f, "Couldn't find a reference to a non detached HEAD"),
            Self::ImposibleUpdate=> write!(f,"Update couldn't be solved by a fast forward."),
        }
    }
}

/// A trait that concerns itself with the "low level" operations of the package
/// manager, with how things are done.
pub trait PMOperations
where
    Self: Sized,
{
    /// A type that implements the `PMDirs` trait
    type Dirs: PMDirs;
    /// A type that implements the `ProjectStore` trait for projects of type `Self::Project`
    type Store: ProjectStore<Self::Project>;
    /// A type that implements the `ProjectT` trait
    type Project: ProjectT;
    /// A type that can hold all the errors originated from the different functions.
    /// It's the same for the `PMOperations` and `PMInteractive`
    type Error: std::error::Error + From<std::io::Error> + From<CommonPMErrors> + From<git2::Error>;
    /// Create a package manager struct, type or w\e
    fn new() -> Result<Self, Self::Error>;
    /// Map the errors created by your store to package manager errors
    /// Typically
    /// ```
    ///Self::Error::Store(err)
    ///```
    fn map_store_error(err: <Self::Store as ProjectStore<Self::Project>>::Error) -> Self::Error;
    /// Map the errors produced by your `PMDirs` implementer to package manager errors
    /// Typically
    /// ```
    ///Self::Error::Dirs(err)
    ///```
    fn map_dir_error(err: <Self::Dirs as PMDirs>::Error) -> Self::Error;
    /// Provide a reference to whatever store you are using.
    /// If you are using a structure to implement the package manager and you
    /// want your package manager to hold within itself a store then its a easy as
    /// ```
    /// &self.store
    /// ```
    fn get_store(&self) -> &Self::Store;
    /// Provide a mutable reference to the same store
    fn get_mut_store(&mut self) -> &mut Self::Store;
    /// Provide a reference to whatever mecanism you are using to implement `PMDirs`
    fn get_dirs(&self) -> &Self::Dirs;
    /// Copy a directory completely from one place to another. With the `fs_extra` crate
    /// this function is as easy as
    ///```
    /// let opts = CopyOptions {
    ///     overwrite: true,
    ///     copy_inside: true,
    ///     ..Default::default()
    /// };
    /// dir::copy(from, to, &opts)?;
    /// Ok(())
    ///
    ///```
    fn copy_directory<T: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: T,
        to: Q,
    ) -> Result<(), Self::Error>;

    /// Clone a project from the projects url
    fn download(&self, prj: &Self::Project) -> Result<(Repository, PathBuf), Self::Error> {
        let git_dir = self.get_dirs().git().join(prj.get_dir());
        let repo = Repository::clone(prj.get_url(), &git_dir)?;
        Ok((repo, git_dir))
    }

    /// change to the branch designated by the project's reference
    fn switch_branch(&self, prj: &Self::Project, repo: &Repository) -> Result<(), Self::Error> {
        let (obj, refe) = repo.revparse_ext(prj.get_ref_string())?;
        repo.checkout_tree(&obj, None)?;
        if let Some(gref) = refe {
            repo.set_head(gref.name().unwrap())?;
        } else {
            Err(CommonPMErrors::BadRef)?;
        }
        Ok(())
    }

    /// Move from wherever to the projects subdirectory in the sources directory
    /// and then build from that directory.
    fn mv_build(&self, prj: &Self::Project, path: &Path) -> Result<(), Self::Error> {
        let src_dir = self.get_dirs().src().join(prj.get_dir());
        if src_dir.exists() {
            std::fs::remove_dir_all(&src_dir)?;
        }
        self.copy_directory(path, &src_dir)?;
        std::fs::remove_dir_all(path)?;
        self.script_runner(prj.get_dir(),prj.get_install())?;
        Ok(())
    }
    /// Update a repo, getting the latest changes if they can be fast forwarded to,
    /// and ensuring that the correct reference is updated
    fn update_repo(&self, prj: &Self::Project, repo: &Repository) -> Result<(), Self::Error> {
        let remotes = repo.remotes()?;
        if !remotes.is_empty() {
            repo.find_remote(remotes.get(0).unwrap_or("origin"))?
                .fetch(&[prj.get_ref_string()], None, None)?;
        }
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date() {
            return Ok(()); // early return
        } else if analysis.0.is_fast_forward() {
            let mut reference = repo.find_reference(prj.get_ref_string())?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(prj.get_ref_string())?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            Err(CommonPMErrors::ImposibleUpdate)?;
        }
        Ok(())
    }
    /// Run a script to install or uninstall a project. It should run it from the
    ///
    fn script_runner(&self, dir: &str, script: &[String]) -> Result<(), Self::Error>;
}

/// A trait that implement the six tasks based on the `PMOperations` trait.
/// It is designed for programtic use of the project
pub trait PMProgrammatic: PMOperations {
    /// Install a project from a known Project in which all parameters are known
    fn install(&mut self, prj: Self::Project) -> Result<(), Self::Error> {
        if !self.get_store().check_unique(prj.get_name(), prj.get_dir()) {
            Err(CommonPMErrors::AlreadyExisting)?;
        }
        let (repo, git_dir) = self.download(&prj)?;
        self.switch_branch(&prj, &repo)?;
        self.get_mut_store()
            .add(prj.clone())
            .map_err(Self::map_store_error)?;
        self.mv_build(&prj, &git_dir)?;
        Ok(())
    }
    /// Uninstall a project given it's name
    fn uninstall(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self.get_one(prj_name)?;
        let dir = &prj.get_dir();
        self.script_runner(dir, prj.get_install())?;
        let src_dir = self.get_dirs().src().join(dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = self.get_dirs().old().join(dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        self.get_mut_store()
            .remove(prj_name)
            .map_err(Self::map_store_error)?;
        Ok(())
    }
    /// Update a project given it's name
    fn update(&self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self.get_one(prj_name)?;
        let dir = prj.get_dir();
        let git_dir = self.get_dirs().git().join(dir);
        let old_dir = self.get_dirs().old().join(dir);
        let src_dir = self.get_dirs().src().join(dir);
        self.copy_directory(&src_dir, &old_dir)?;
        self.copy_directory(&src_dir, &git_dir)?;
        let repo = Repository::open(&git_dir)?;
        self.switch_branch(prj, &repo)?;
        self.update_repo(prj, &repo)?;
        self.mv_build(prj, &git_dir)?;
        Ok(())
    }
    /// Install the older version of a project given it's name
    fn restore(&self, prj_name: &str) -> Result<(), Self::Error> {
        let prj = self.get_one(prj_name)?;
        let dir = prj.get_dir();
        let old_dir = self.get_dirs().old().join(dir);
        let src_dir = self.get_dirs().src().join(dir);
        std::fs::remove_dir_all(&src_dir)?;
        self.copy_directory(&old_dir, &src_dir)?;
        self.script_runner(prj.get_dir(), prj.get_install())?;
        Ok(())
    }
    ///
    fn edit(&mut self, prj_name: &str, prj: Self::Project) -> Result<(), Self::Error> {
        self.get_mut_store()
            .edit(prj_name, prj)
            .map_err(Self::map_store_error)?;
        Ok(())
    }
    /// Get the project configuration, given it's name
    fn get_one(&self, prj_name: &str) -> Result<&Self::Project, Self::Error>{
        Ok(self.get_store().get_ref(prj_name).ok_or(CommonPMErrors::NonExisting)?)
    }
    /// Get a list of references to all the projects in storage
    fn get_all(&self)-> Vec<&Self::Project>{
        self.get_store().iter().collect()
    }
}

/// This trait defines methods for the six tasks to be interactive, it provides
/// none of the methods, as different implementors will bring different preferences
/// for dependencies and ways of interacting with the user.
pub trait PMInteractive: PMProgrammatic {
    /// With this function a project should be downloaded, ask for the necessary
    /// information and then build and install itself
    /// The contents could look something like:
    /// ```
    /// let project : Project{
    ///     directory : somehow_get_directory()
    ///     url: url
    ///     ..Default::default()
    /// };
    /// let (repo, git_dir) = self.download(&project)?;
    /// let project = somehow_get_rest_of project()?;
    /// self.switch_branch(&project, &repo)?;
    /// self.get_mut_store()
    ///     .add(project.clone())
    ///     .map_err(Self::map_store_error)?;
    /// self.build_rm(&project, &git_dir)?;
    /// Ok(())
    /// ```
    fn i_install(&mut self, url: &str) -> Result<(), Self::Error>;
    /// This function should give the available information of the projects
    fn i_list(&self, prj_names:&[&str]) -> Result<(), Self::Error>;
    /// Edit a projects information and store that
    fn i_edit(&mut self, package: &str) -> Result<(), Self::Error>;
    /// Update the projects, (Possibly a forwarding of the `PMBasics` update
    /// method applied to each of the packages)
    fn i_update(&self, prj_names: &[&str]) -> Result<(), Self::Error>;
    /// Take an the last version of a project, set it as the current and build
    /// and install it (Possibly just a forwarding of the `PMBasics` restore method)
    fn i_restore(self, prj_names: &[&str]) -> Result<(), Self::Error>;
    /// Uninstall a project and delete the related information that the
    /// package manager has about it. (Possibly a forwarding of the `PMBasics` uninstall method)
    fn i_uninstall(self, prj_names: &[&str]) -> Result<(), Self::Error>;
}
