//! This module defines the trait that enables getting build suggestions.
//!
//! It also defines a struct that implements the trait and the auxiliary
//! structs and functions that are needed for that.

use regex::Regex;
use std::path::PathBuf;
mod origins;
use origins::SuggestionsTable;
mod error;
use agpm_abstract::PMDirs;
use agpm_interactions::Suggester;
pub use error::SuggestionsError;
use std::marker::PhantomData;

/// A structure that implements the `BuildSuggester` trait
pub struct Suggestions<D: PMDirs> {
    install: Vec<Vec<String>>,
    uninstall: Vec<Vec<String>>,
    dirs: PhantomData<D>,
}

impl<D: PMDirs> Suggestions<D> {
    /// docu
    pub fn get_build_suggestions(
        readme_file: &PathBuf,
    ) -> Result<Vec<Vec<String>>, SuggestionsError<D::Error>> {
        let regex = Regex::new(r"((?i)compil|instal|build)").unwrap();
        markdown_extract::extract_from_path(readme_file, &regex).map_err(|e| e.into())
    }
}

impl<D: PMDirs> Suggester for Suggestions<D> {
    type Error = SuggestionsError<D::Error>;
    fn new(name: &str) -> Result<Self, Self::Error> {
        let path = D::new().map_err(Self::Error::Dirs)?.git().join(name);
        let mut readme: Vec<Vec<String>> = vec![];
        for each in glob::glob(path.join("*.md").to_str().ok_or(SuggestionsError::Path)?)? {
            readme.append(&mut Self::get_build_suggestions(&each?).unwrap_or_default());
        }
        match SuggestionsTable::<D>::new() {
            Ok(db) => {
                let db_sug = db.get_suggestions(&path);
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
                    dirs: PhantomData::default(),
                })
            }
            Err(_) => Ok(Self {
                install: readme,
                uninstall: vec![],
                dirs: PhantomData::default(),
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
