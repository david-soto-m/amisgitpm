#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// What to do when updating a project
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Default)]
pub enum UpdatePolicy {
    /// Update the project to the newest version every time
    Always,
    /// Ask whether to update or not
    Ask,
    /// Do not update the repo, **default** value
    #[default]
    Never,
}

impl std::fmt::Display for UpdatePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => {
                write!(f, "Always try to update the project")
            }
            Self::Ask => {
                write!(f, "Ask wether ot update or not")
            }
            Self::Never => {
                write!(f, "Never try to update the project")
            }
        }
    }
}

/// The structure that all installed projects must have
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Project {
    /// The name of the projects
    pub name: String,
    /// The name of the directory in which the project is going to be stored
    pub dir: String,
    /// The url from which to git clone the project, it can be a file url
    pub url: String,
    /// A string to identify the branch which you want installed
    pub ref_string: String,
    /// Whether to update, ask or never update the project
    pub update_policy: UpdatePolicy,
    /// How to install the project. The elements are joined with && before execution
    pub install_script: Vec<String>,
    /// How to uninstall the project. The elements are joined with && before execution
    pub uninstall_script: Vec<String>,
}

/// How to interact with however your projects are stored
/// The idea is that you can implement this trait with any technology you want
/// to use. Any kind of database, a xml document, a collection of json docs...
pub trait ProjectStore
where
    Self: Sized,
{
    /// Your custom Error type
    type Error: std::error::Error + 'static;
    /// A function to start up your store
    fn new() -> Result<Self, Self::Error>;
    /// Add an item to the store
    fn add(&mut self, prj: Project) -> Result<(), Self::Error>;
    /// Remove an item from the store
    fn remove(&mut self, prj_name: &str) -> Result<(), Self::Error>;
    /// Get a reference to an item inside the store
    fn get_ref<'a>(&'a self, prj_name: &str) -> Option<&'a Project>;
    /// Return a cloned instance of a project in the store
    fn get_clone(&self, prj_name: &str) -> Option<Project>;
    /// Replace the project that used to go by the old_prj_name name with the new_prj item
    fn edit(&mut self, old_prj_name: &str, new_prj: Project) -> Result<(), Self::Error> {
        self.remove(old_prj_name)?;
        self.add(new_prj)?;
        Ok(())
    }
    /// If a directory name is free for use
    fn check_dir_free(&self, dir: &str) -> bool;
    /// If a name is free for use
    fn check_name_free(&self, prj_name: &str) -> bool;
    /// check if a combination of directory and name are both free for use
    fn check_unique(&self, prj_name: &str, dir: &str) -> bool {
        self.check_dir_free(dir) && self.check_name_free(prj_name)
    }
    /// Return an iterator over refereneces of Project Items
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Project> + 'a>;
    /// Check if there are elements in the store
    fn is_empty(&self) -> bool;
}
