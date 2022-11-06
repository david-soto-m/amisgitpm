//! This module defines the trait that enables getting build suggestions.
//!
//! It also defines a struct that implements the trait and the auxiliary
//! structs and functions that are needed for that.
use crate::error::SuggestionsError;
use agpm_abstract::PMDirs;
use regex::Regex;
use std::path::{Path, PathBuf};

/// A structure that implements the `BuildSuggester` trait
pub struct Suggestions {
    install: Vec<Vec<String>>,
    uninstall: Vec<Vec<String>>,
}

impl Suggestions {
    pub fn get_build_suggestions(
        readme_file: &PathBuf,
    ) -> Result<Vec<Vec<String>>, SuggestionsError> {
        let regex = Regex::new(r"((?i)compil|instal|build)").unwrap();
        markdown_extract::extract_from_path(readme_file, &regex).map_err(|e| e.into())
    }
    pub fn new<T: PMDirs>(for_: impl AsRef<Path>) -> Result<Self, SuggestionsError> {
        let from = T::new()
            .map_err(|e| SuggestionsError::DirsError(e.to_string()))?
            .projects_db()
            .parent()
            .unwrap()
            .join("suggestions");
        let mut readme: Vec<Vec<String>> = vec![];
        for each in glob::glob(
            for_.as_ref()
                .join("*.md")
                .to_str()
                .ok_or(SuggestionsError::Path)?,
        )? {
            readme.append(&mut Self::get_build_suggestions(&each?).unwrap_or_default());
        }
        match SuggestionsTable::new(from.as_ref()) {
            Ok(db) => {
                let db_sug = db.get_suggestions(for_.as_ref());
                Ok(Self {
                    install: db_sug
                        .iter()
                        .flat_map(|&e| e.install_suggestions.clone())
                        .chain(readme)
                        .collect(),
                    uninstall: db_sug
                        .iter()
                        .flat_map(|e| e.uninstall_suggestions.clone())
                        .collect(),
                })
            }
            Err(_) => Ok(Self {
                install: readme,
                uninstall: vec![],
            }),
        }
    }
    pub fn get_install(&self) -> &Vec<Vec<String>> {
        &self.install
    }
    pub fn get_uninstall(&self) -> &Vec<Vec<String>> {
        &self.uninstall
    }
}

use json_tables::{Deserialize, Serialize, Table, TableError};
/// This function examines a given markdown file for headers that matches with
/// the case insensitive regex `(compil|instal|build)`
/// A structure that holds the information needed to detect and suggest
/// some build instructions
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SuggestionsItem {
    /// File names to detect in order to make the suggestions contained bellow
    pub file_types: Vec<String>,
    /// The set of suggestions in order to build and install a project
    pub install_suggestions: Vec<Vec<String>>,
    /// The set of suggestion in order to uninstall a project
    pub uninstall_suggestions: Vec<Vec<String>>,
}

pub struct SuggestionsTable {
    pub table: Table<SuggestionsItem>,
}

impl SuggestionsTable {
    pub fn new(dir: &Path) -> Result<Self, TableError> {
        Ok(Self {
            table: Table::builder(dir).set_read_only().load()?,
        })
    }
    pub fn get_suggestions(&self, path: &Path) -> Vec<&SuggestionsItem> {
        self.table
            .get_table_content()
            .filter_map(|e| {
                for pattern in &e.info.file_types {
                    if glob::glob(path.join(pattern).to_str()?)
                        .ok()?
                        .next()
                        .is_some()
                    {
                        return Some(&e.info);
                    }
                }
                None
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::SuggestionsTable;
    use std::path::Path;
    #[test]
    fn makes_suggestions() {
        let db_loc = Path::new("suggestions");
        let table = SuggestionsTable::new(db_loc).unwrap();
        let len = table
            .get_suggestions(&Path::new("../tests/projects/mess_project"))
            .len();
        assert_eq!(len, 3);
    }
    #[test]
    fn all_build_aux_json_is_correct() {
        let db_loc = Path::new("suggestions");
        SuggestionsTable::new(db_loc).unwrap();
    }
    use std::path::PathBuf;
    #[test]
    fn gets_different_sections() {
        let sugg = super::Suggestions::new();
        let hx = sugg
            .get_build_suggestions(&PathBuf::from("tests/mdowns/Helix.md"))
            .unwrap();
        assert_eq!(hx[0].len(), 48);
        let swave = sugg
            .get_build_suggestions(&PathBuf::from("tests/mdowns/Shortwave.md"))
            .unwrap();
        assert_eq!(swave.len(), 2);
        assert_eq!(swave[0].len(), 10);
        assert_eq!(swave[1].len(), 26);
    }
}
