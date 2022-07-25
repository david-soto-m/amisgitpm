//! This module contains the structure that stores build suggestions
//! BuildAux, and the implementation for a table of such structures

use crate::dbmanager::{Permissions, Table, TableError};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{DirEntry, ReadDir};

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

/// Special functions for Tables of BuildAux structures, such as loading the tables
/// or getting the suggestions for a repo.
impl Table<BuildAux> {
    /// Get the table of pre-made suggestions for compilations.
    pub async fn get_table() -> Result<Table<BuildAux>, TableError> {
        Table::load("db/build_aux", Permissions::ReadOnly).await
    }
    /// Get the build suggestions from the table for the files examined in a directory
    ///```ignore
    /// let project_dir = std::fs::read_dir("tests/projects/mess_project")?;
    /// Table::<BuildAux>::get_table().await?.get_suggestions(project_dir);
    ///```
    ///
    /// It doesn't panic, but ignores all errors, so it might return empty without
    /// information about why in cases in which it ought to return with something
    pub async fn get_suggestions(&mut self, files: ReadDir) -> Vec<&BuildAux> {
        let info_vec: Vec<&BuildAux> = self.get_info_iter().collect();

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
                    info_vec.par_iter().find_any(|b_aux| {
                        b_aux.file_types.par_iter().any(|file_hint| {
                            match f_entry.file_name().into_string() {
                                Ok(f_entry_name) => *file_hint == f_entry_name,
                                Err(_) => false,
                            }
                        })
                    }).copied()
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::dbmanager::Table;
    use crate::dbs::BuildAux;
    use std::fs;
    #[tokio::test]
    async fn makes_suggestions() {
        assert_eq!(
            Table::<BuildAux>::get_table()
                .await
                .unwrap()
                .get_suggestions(fs::read_dir("tests/projects/mess_project").unwrap())
                .await
                .len(),
            3
        );
    }
    #[tokio::test]
    async fn all_build_aux_json_is_correct() {
        Table::<BuildAux>::get_table().await.unwrap();
        assert!(true)
    }
}
