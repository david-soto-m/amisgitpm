use amisgitpm::{ProjectStore, ProjectT};
use git2::Repository;
use std::path::Path;

pub trait Interactions<P: ProjectT, PS: ProjectStore<P>>: Sized {
    type Error: std::error::Error;
    fn new() -> Result<Self, Self::Error>;
    fn url_to_download(&self, url: &str) -> Result<P, Self::Error>;
    fn repo_to_checkout_branch(&self, prj: P, repo: &Repository) -> Result<P, Self::Error>;
    fn create_project(&self, prj_stub: &P, store: &PS, wher: &Path) -> Result<P, Self::Error>;
    fn edit(&self, prj: P) -> Result<P, Self::Error>;
    fn list(&self, store: &PS) -> Result<(), Self::Error>;
    fn list_one(&self, prj: &P) -> Result<(), Self::Error>;
    fn update_confirm(&self, prj: &P) -> bool;
}
