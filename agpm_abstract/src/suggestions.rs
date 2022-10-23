use std::path::Path;

/// A trait that standardizes how to provide build suggestions for the install process
pub trait Suggester
where
    Self: Sized,
{
    /// An error for new operations
    type Error: std::error::Error;
    /// The declaration of a new structure that implements the trait
    fn new(path: &Path) -> Result<Self, Self::Error>;
    /// Get a reference to a list of install suggestions, these being a list of strings
    fn get_install(&self) -> &Vec<Vec<String>>;
    /// Get a reference to a list of uninstall suggestions, these being a list of strings
    fn get_uninstall(&self) -> &Vec<Vec<String>>;
}
