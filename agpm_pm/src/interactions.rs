use amisgitpm::{ProjectIface, ProjectStore};
use git2::Repository;
use std::path::Path;

/// A trait to separate the implementation of the interactions from the actual
// interactive functions
pub trait Interactions<P: ProjectIface, PS: ProjectStore<P>>: Sized {
    /// The error that interactions will return
    type Error: std::error::Error;
    /// Create an Interactions implementor
    fn new() -> Result<Self, Self::Error>;
    /// Go from an url to a downloadable project -> A project with `url`, and `dir`
    fn url_to_download(&self, url: &str) -> Result<P, Self::Error>;
    /// Go from a git repo and obtain a valid reference string for the branch to be used
    fn repo_to_checkout_branch(&self, prj: P, repo: &Repository) -> Result<P, Self::Error>;
    /// Complete a project that starts with a `url`, a `ref_string` and a name
    fn create_project(&self, prj_stub: &P, store: &PS, wher: &Path) -> Result<P, Self::Error>;
    /// Interactively edit a project
    fn edit(&self, prj: P) -> Result<P, Self::Error>;
    /// List all projects
    fn list(&self, store: &PS) -> Result<(), Self::Error>;
    /// Give details about one project
    fn list_one(&self, prj: &P) -> Result<(), Self::Error>;
    /// Confirm whether to update
    fn update_confirm(&self, prj: &P) -> bool;
}
