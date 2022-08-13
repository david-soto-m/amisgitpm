//! This module contains the structure that stores build suggestions
//! DBSuggestions, and the implementation for a table of such structures

use crate::dirutils;
use json_tables::Table;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::ReadDir;

use crate::build_suggestions::SuggestionsError;
/// A structure that holds the information needed to detect and suggest
/// some build instructions
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DBSuggestions {
    /// File names to detect in order to make the suggestions contained bellow
    pub file_types: Vec<String>,
    /// The set of suggestions in order to build & install a project
    pub install_suggestions: Vec<Vec<String>>,
    /// The set of suggestion in order to uninstall a project
    pub uninstall_suggestions: Vec<Vec<String>>,
}

pub struct DBSuggestionsTable {
    pub table: Table<DBSuggestions>,
}

/// Special functions for Tables of DBSuggestions structures, such as loading the tables
/// or getting the suggestions for a repo.
impl DBSuggestionsTable {
    /// Get the table of pre-made suggestions for compilations.
    pub fn new() -> Result<Self, SuggestionsError> {
        Ok(Self {
            table: Table::builder(dirutils::suggestions_db())
                .set_read_only()
                .load()?,
        })
    }
    /// Get the build suggestions from the table for the files examined in a directory
    ///```ignore
    /// let project_dir = std::fs::read_dir("tests/projects/mess_project")?;
    /// Table::<DBSuggestions>::get_table().await?.get_suggestions(project_dir);
    ///```
    ///
    /// It doesn't panic, but ignores all errors, so it might return empty without
    /// information about why in cases in which it ought to return with something
    pub fn get_suggestions(&self, files: ReadDir) -> Vec<&DBSuggestions> {
        let info: Vec<&DBSuggestions> = self.table.get_info_iter().collect();
        files
            .par_bridge()
            .filter_map(|file| {
                let f_entry = match file {
                    Ok(file) => file,
                    Err(_) => return None,
                };
                let is_file = match f_entry.file_type() {
                    Ok(ftype) => ftype,
                    Err(_) => return None,
                }
                .is_file();
                if is_file {
                    info.par_iter()
                        .find_any(|b_aux| {
                            b_aux.file_types.par_iter().any(|file_hint| {
                                match f_entry.file_name().into_string() {
                                    Ok(f_entry_name) => *file_hint == f_entry_name,
                                    Err(_) => false,
                                }
                            })
                        })
                        .copied()
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::build_suggestions::DBSuggestionsTable;
    use std::fs;
    #[test]
    fn makes_suggestions() {
        let table = DBSuggestionsTable::new().unwrap();
        let len = table
            .get_suggestions(fs::read_dir("tests/projects/mess_project").unwrap())
            .len();

        assert_eq!(len, 3);
    }
    #[tokio::test]
    async fn all_build_aux_json_is_correct() {
        DBSuggestionsTable::new().unwrap();
        assert!(true)
    }
}
