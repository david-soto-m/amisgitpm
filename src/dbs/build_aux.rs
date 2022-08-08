//! This module contains the structure that stores build suggestions
//! BuildAux, and the implementation for a table of such structures

use json_tables::{Table, TableError};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::ReadDir;

/// A structure that holds the information needed to detect and suggest
/// some build instructions
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildAux {
    /// File names to detect in order to make the suggestions contained bellow
    pub file_types: Vec<String>,
    /// The set of suggestions in order to build & install a project
    pub install_suggestions: Vec<Vec<String>>,
    /// The set of suggestion in order to uninstall a project
    pub uninstall_suggestions: Vec<Vec<String>>,
}

pub struct BuildAuxTable {
    pub table: Table<BuildAux>,
}

/// Special functions for Tables of BuildAux structures, such as loading the tables
/// or getting the suggestions for a repo.
impl BuildAuxTable {
    /// Get the table of pre-made suggestions for compilations.
    pub fn new() -> Result<Self, TableError> {
        Ok(Self {
            table: Table::builder("db/build_aux").set_read_only().load()?,
        })
    }
    /// Get the build suggestions from the table for the files examined in a directory
    ///```ignore
    /// let project_dir = std::fs::read_dir("tests/projects/mess_project")?;
    /// Table::<BuildAux>::get_table().await?.get_suggestions(project_dir);
    ///```
    ///
    /// It doesn't panic, but ignores all errors, so it might return empty without
    /// information about why in cases in which it ought to return with something
    pub async fn get_suggestions(&self, files: ReadDir) -> Vec<&BuildAux> {
        let info: Vec<&BuildAux> = self.table.get_info_iter().collect();
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
    use crate::dbs::BuildAuxTable;
    use std::fs;
    #[tokio::test]
    async fn makes_suggestions() {
        let table = BuildAuxTable::new().unwrap();
        let len = table
            .get_suggestions(fs::read_dir("tests/projects/mess_project").unwrap())
            .await
            .len();

        assert_eq!(len, 3);
    }
    #[tokio::test]
    async fn all_build_aux_json_is_correct() {
        BuildAuxTable::new().unwrap();
        assert!(true)
    }
}
