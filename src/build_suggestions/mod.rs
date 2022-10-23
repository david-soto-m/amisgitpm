#![warn(missing_docs)]

//! This module defines the trait that enables getting build suggestions.
//!
//! It also defines a struct that implements the trait and the auxiliary
//! structs and functions that are needed for that.

use glob;
use std::path::Path;

mod db_suggestions;
use db_suggestions::SuggestionsTable;
mod error;
mod mdown;
use amisgitpm_types_traits::Suggester;
pub use error::SuggestionsError;

/// A structure that implements the `BuildSuggester` trait
pub struct BuildSuggestions {
    install: Vec<Vec<String>>,
    uninstall: Vec<Vec<String>>,
}

impl Suggester for BuildSuggestions {
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
