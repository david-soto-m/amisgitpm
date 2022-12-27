//! This crate defines the traits for a project manager.
//!
//! An amisgitpm compliant project manager must do six tasks
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
//! the procedures to make the project manager work. Most of the methods that
//! are not provided are very straight forward to implement.
//!
//! The API facing traits is `PMProgramatic` It is completely provided for you. It
//! does the six tasks
//!
//! The "User" facing traits is `PMInteractive`.
//! All methods must be implemented, but will frequently just run methods from
//! `PMProgramatic` or reimplement some other using `PMOperations`
//!

use crate::{Directories, ProjectIface, ProjectStore};
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
            Self::NonExisting => write!(f, "That project that doesn't exist!"),
            Self::Os2Str => write!(f, "Couldn't convert from &Osstr to utf-8 &str"),
            Self::BadRef => write!(f, "Couldn't find a reference to a non detached HEAD"),
            Self::ImposibleUpdate => write!(f, "Update couldn't be solved by a fast forward."),
        }
    }
}

/// A trait that concerns itself with the "low level" operations of the project
/// manager, with how things are done.
pub trait PMOperations
where
    Self: Sized,
{
    /// A type that implements the `PMDirs` trait
    type Dirs: Directories;
    /// A type that implements the `ProjectStore` trait for projects of type `Self::Project`
    type Store: ProjectStore<Self::Project>;
    /// A type that implements the `ProjectT` trait
    type Project: ProjectIface;
    /// A type that can hold all the errors originated from the different functions.
    /// It's the same for the `PMOperations` and `PMInteractive`
    type Error: std::error::Error + From<std::io::Error> + From<CommonPMErrors> + From<git2::Error>;
    /// Create a project manager struct, type or whatever
    fn new() -> Result<Self, Self::Error>;
    /// Map the errors created by your store to project manager errors
    /// Typically
    /// ```ignore
    ///Self::Error::Store(err)
    ///```
    fn map_store_error(err: <Self::Store as ProjectStore<Self::Project>>::Error) -> Self::Error;
    /// Map the errors produced by your `PMDirs` implementer to project manager errors
    /// Typically
    /// ```ignore
    ///Self::Error::Dirs(err)
    ///```
    fn map_dir_error(err: <Self::Dirs as Directories>::Error) -> Self::Error;
    /// Provide a reference to whatever store you are using.
    /// If you are using a structure to implement the project manager and you
    /// want your project manager to hold within itself a store then its a easy as
    /// ```ignore
    /// &self.store
    /// ```
    fn get_store(&self) -> &Self::Store;
    /// Provide a mutable reference to the same store
    fn get_mut_store(&mut self) -> &mut Self::Store;
    /// Provide a reference to whatever mechanism you are using to implement `Directories`
    fn get_dirs(&self) -> &Self::Dirs;
    /// Copy a directory completely from one place to another. With the `fs_extra` crate
    /// this function is as easy as
    ///```ignore
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
    /// # Errors
    /// - Failure cloning the repo.
    fn download(&self, prj: &Self::Project) -> Result<(Repository, PathBuf), Self::Error> {
        let git_dir = self.get_dirs().git().join(prj.get_dir());
        let repo = Repository::clone(prj.get_url(), &git_dir)?;
        Ok((repo, git_dir))
    }

    /// Change to the branch designated by the project's reference
    /// # Errors
    /// - Finding the object `prj.get_ref_string()` in the repo
    /// - Checking out the tree with that object
    /// - If there is no reference to set the head to -> `CommonPMErrors::BadRef`
    /// - Setting the head to the head of the reference
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
    /// # Errors
    /// - Deleting the directories (in established in `Dirs::new().unwrap().src` or `path`)
    /// - Copying the directories
    fn mv(&self, prj: &Self::Project, path: &Path) -> Result<(), Self::Error> {
        let src_dir = self.get_dirs().src().join(prj.get_dir());
        if src_dir.exists() {
            std::fs::remove_dir_all(&src_dir)?;
        }
        self.copy_directory(path, &src_dir)?;
        std::fs::remove_dir_all(path)?;
        Ok(())
    }
    /// Run the build script from the `src()` directory.
    /// # Errors
    /// - Script runner failure
    fn build(&self, prj: &Self::Project) -> Result<(), Self::Error> {
        self.script_runner(prj.get_dir(), prj.get_install())
    }
    /// Run the uninstall script from the `src()` directory
    /// # Errors
    /// - Script runner failure
    fn unbuild(&self, prj: &Self::Project) -> Result<(), Self::Error> {
        self.script_runner(prj.get_dir(), prj.get_uninstall())
    }
    /// Update a repo, getting the latest changes if they can be fast forwarded to,
    /// and ensuring that the correct reference is updated. If any updates have
    /// been applied returns True else false
    /// # Errors
    /// - Getting the remotes
    /// - Fetching the remotes
    /// - Finding the reference `"FETCH_HEAD"`
    /// - Getting the commit to said reference
    /// - Analyzing the merge
    /// - If there is no possibility of solving with Fast Forward, then -> `CommonPMErrors::ImposibleUpdate`
    /// - Resolving the merge with Fast-Forward strategy
    /// - Seting the head to the new head
    fn update_repo(&self, prj: &Self::Project, repo: &Repository) -> Result<bool, Self::Error> {
        let remotes = repo.remotes()?;
        if !remotes.is_empty() {
            repo.find_remote(remotes.get(0).unwrap_or("origin"))?
                .fetch(&[prj.get_ref_string()], None, None)?;
        }
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date() {
            return Ok(false); // early return
        } else if analysis.0.is_fast_forward() {
            let mut reference = repo.find_reference(prj.get_ref_string())?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(prj.get_ref_string())?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            Err(CommonPMErrors::ImposibleUpdate)?;
        }
        Ok(true)
    }
    /// Run a script to install or uninstall a project
    fn script_runner<T: AsRef<str>, Q: AsRef<[T]>>(
        &self,
        dir: &str,
        script: Q,
    ) -> Result<(), Self::Error>;
}

/// A trait that implement the six tasks based on the `PMOperations` trait.
/// It is designed for programtic use of the project
pub trait PMProgrammatic: PMOperations {
    /// Install a project from a known Project in which all parameters are known
    /// # Errors
    /// - If there is a project with that name or directory already in use -> `CommonPMErrors::AlreadyExisting`
    /// - Switching branches
    /// - Moving dirs
    /// - Adding to the store
    /// - Building the project
    fn install(&mut self, prj: Self::Project) -> Result<(), Self::Error> {
        if !self.get_store().check_unique(prj.get_name(), prj.get_dir()) {
            Err(CommonPMErrors::AlreadyExisting)?;
        }
        let (repo, git_dir) = self.download(&prj)?;
        self.switch_branch(&prj, &repo)?;
        self.mv(&prj, &git_dir)?;
        self.get_mut_store()
            .add(prj.clone())
            .map_err(Self::map_store_error)?;
        self.build(&prj)?;
        Ok(())
    }
    /// Uninstall a project given it's name
    /// # Errors
    /// - Unable to get the project -> `CommonPMErrors::NonExisting`
    /// - Unable to run the uninstall script
    /// - Unable to delete directories -> Normally permissions errors.
    /// - Unable to remove from store.
    fn uninstall<T: AsRef<str>>(&mut self, prj_name: T) -> Result<(), Self::Error> {
        let prj = self
            .get_one(prj_name.as_ref())
            .ok_or(CommonPMErrors::NonExisting)?;
        let dir = &prj.get_dir();
        self.unbuild(prj)?;
        let src_dir = self.get_dirs().src().join(dir);
        std::fs::remove_dir_all(src_dir)?;
        let old_dir = self.get_dirs().old().join(dir);
        if old_dir.exists() {
            std::fs::remove_dir_all(old_dir)?;
        }
        self.get_mut_store()
            .remove(prj_name.as_ref())
            .map_err(Self::map_store_error)?;
        Ok(())
    }
    /// Update a project given it's name
    /// # Errors
    /// - Unable to get the project -> `CommonPMErrors::NonExisting`
    /// - Unable to copy directories
    /// - Unable to open the repo
    /// - Unable to switch to the established branch
    /// - Unable to update the repo
    /// - Unable to move the project
    /// - Unable to build the project
    fn update<T: AsRef<str>>(&self, prj_name: T) -> Result<(), Self::Error> {
        let prj = self
            .get_one(prj_name.as_ref())
            .ok_or(CommonPMErrors::NonExisting)?;
        let dir = prj.get_dir();
        let git_dir = self.get_dirs().git().join(dir);
        let old_dir = self.get_dirs().old().join(dir);
        let src_dir = self.get_dirs().src().join(dir);
        self.copy_directory(&src_dir, &old_dir)?;
        self.copy_directory(&src_dir, &git_dir)?;
        let repo = Repository::open(&git_dir)?;
        self.switch_branch(prj, &repo)?;
        if self.update_repo(prj, &repo)?{
            self.mv(prj, &git_dir)?;
            self.build(prj)?;
        } else {
            std::fs::remove_dir_all(git_dir)?;
        }
        Ok(())
    }
    /// Install the older version of a project given it's name
    /// # Errors
    /// - Unable to get the project -> `CommonPMErrors::NonExisting`
    /// - Unable to remove the src directory
    /// -  Unable to copy the directory from old to new
    /// - Unable to build the project
    fn restore<T: AsRef<str>>(&self, prj_name: T) -> Result<(), Self::Error> {
        let prj = self
            .get_one(prj_name.as_ref())
            .ok_or(CommonPMErrors::NonExisting)?;
        let dir = prj.get_dir();
        let old_dir = self.get_dirs().old().join(dir);
        let src_dir = self.get_dirs().src().join(dir);
        std::fs::remove_dir_all(&src_dir)?;
        self.copy_directory(old_dir, &src_dir)?;
        self.build(prj)?;
        Ok(())
    }
    /// Substitute the contents of a project with name `prj_name` with the contents in `prj`
    /// # Errors
    /// - Store error getting the project or substituting it.
    fn edit<T: AsRef<str>>(&mut self, prj_name: T, prj: Self::Project) -> Result<(), Self::Error> {
        self.get_mut_store()
            .edit(prj_name.as_ref(), prj)
            .map_err(Self::map_store_error)?;
        Ok(())
    }
    /// Get the project configuration, given it's name
    fn get_one<T: AsRef<str>>(&self, prj_name: T) -> Option<&Self::Project> {
        self.get_store().get_ref(prj_name.as_ref())
    }
    /// Get a list of references to all the projects in storage
    fn get_all(&self) -> Vec<&Self::Project> {
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
    /// ```ignore
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
    /// self.mv(&project, &git_dir)?;
    /// self.build(&project)?;
    /// Ok(())
    /// ```
    fn i_install<T: AsRef<str>>(&mut self, url: T) -> Result<(), Self::Error>;
    /// This function should give the available information of the projects
    /// If the list is empty it's suggested to print all the projects information
    fn i_list<T: AsRef<str>, Q: AsRef<[T]>>(&self, prj_names: Q) -> Result<(), Self::Error>;
    /// Edit a projects information and store that
    fn i_edit<T: AsRef<str>>(&mut self, project: T) -> Result<(), Self::Error>;
    /// Update the projects, (Possibly a forwarding of the `PMBasics` update
    /// method applied to each of the projects)
    fn i_update<T: AsRef<str>, Q: AsRef<[T]>>(&self, prj_names: Q) -> Result<(), Self::Error>;
    /// Take an the last version of a project, set it as the current and build
    /// and install it (Possibly just a forwarding of the `PMBasics` restore method)
    fn i_restore<T: AsRef<str>, Q: AsRef<[T]>>(self, prj_names: Q) -> Result<(), Self::Error>;
    /// Uninstall a project and delete the related information that the
    /// project manager has about it. (Possibly a forwarding of the `PMBasics` uninstall method)
    fn i_uninstall<T: AsRef<str>, Q: AsRef<[T]>>(
        &mut self,
        prj_names: Q,
    ) -> Result<(), Self::Error>;
}
