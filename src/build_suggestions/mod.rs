#![warn(missing_docs)]

//! This module defines the trait that enables getting build suggestions.
//!
//! It also defines a struct that implements the trait and the auxiliary
//! structs and functions that are needed for that.

use glob;
use std::path::Path;

mod db_suggestions;
use db_suggestions::SuggestionsTable;
mod mdown;
mod error;
pub use error::SuggestionsError;

/// A structure that implements the `BuildSuggester` trait
pub struct BuildSuggestions {
    install: Vec<Vec<String>>,
    uninstall: Vec<Vec<String>>,
}

impl BuildSuggester for BuildSuggestions {
    type Error = SuggestionsError;
    fn new(path: &Path) -> Result<Self, SuggestionsError> {
        let mut readme: Vec<Vec<String>> = vec![];
        for each in glob::glob(path.join("*.md").to_str().ok_or(SuggestionsError::Path)?)? {
            readme.append(&mut mdown::get_build_suggestions(&each?).unwrap_or_default());
        }
        match SuggestionsTable::new() {
            Ok(db) => {
                let db_sug = db.get_suggestions(path);
                Ok(Self {
                    install: db_sug
                        .iter()
                        .flat_map(|&e| e.install_suggestions.to_owned())
                        .chain(readme)
                        .collect(),
                    uninstall: db_sug
                        .iter()
                        .flat_map(|e| e.uninstall_suggestions.to_owned())
                        .collect(),
                })
            }
            Err(_) => Ok(Self {
                install: readme,
                uninstall: vec![],
            }),
        }
    }
    fn get_install(&self) -> &Vec<Vec<String>> {
        &self.install
    }
    fn get_uninstall(&self) -> &Vec<Vec<String>> {
        &self.uninstall
    }
}

/// A trait that standardizes how to provide build suggestions for the install process
pub trait BuildSuggester
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
