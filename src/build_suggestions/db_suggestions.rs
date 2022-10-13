use crate::dirutils::{PMDirs, PMDirsImpl};
use glob;
use json_tables::Table;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::build_suggestions::SuggestionsError;
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

/// A wrapper structure for a json_tables of `SuggestionsItem`
///
/// It allows for some extra methods to be defined
pub struct SuggestionsTable {
    /// The table that holds the different suggestions for projects
    pub table: Table<SuggestionsItem>,
}

/// Special functions for Tables of DBSuggestions structures, such as loading the tables
/// or getting the suggestions for a repo.
impl SuggestionsTable {
    /// Get the table of pre-made suggestions for compilations.
    pub fn new() -> Result<Self, SuggestionsError> {
        Ok(Self {
            table: Table::builder(PMDirsImpl::new().suggestions_db())
                .set_read_only()
                .load()?,
        })
    }
    /// Get the build suggestions from the table for the files examined in a directory
    ///```ignore
    /// let table = amisgitpm::build_suggestions::SuggestionsTable::new().unwrap();
    /// let sugg = table.get_suggestions(&std::path::Path::new("tests/projects/mess_project"));
    ///```
    ///
    /// It doesn't panic, but ignores all errors, so it might return empty without
    /// information about why in cases in which it ought to return with something
    pub fn get_suggestions(&self, path: &Path) -> Vec<&SuggestionsItem> {
        self.table
            .get_table_content()
            .filter_map(|e| {
                for pattern in &e.info.file_types {
                    for _ in glob::glob(path.join(&pattern).to_str()?).ok()? {
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
    use crate::build_suggestions::SuggestionsTable;
    use std::path::Path;
    #[test]
    fn makes_suggestions() {
        let table = SuggestionsTable::new().unwrap();
        let len = table
            .get_suggestions(&Path::new("tests/projects/mess_project"))
            .len();

        assert_eq!(len, 3);
    }
    #[test]
    fn all_build_aux_json_is_correct() {
        SuggestionsTable::new().unwrap();
        assert!(true)
    }
}
