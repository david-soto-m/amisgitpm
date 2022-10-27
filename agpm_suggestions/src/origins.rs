use std::{path::Path, marker::PhantomData};
use agpm_abstract::PMDirs;
use json_tables::{Table, Serialize, Deserialize};
use crate::SuggestionsError;
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

pub struct SuggestionsTable<D: PMDirs> {
    pub table: Table<SuggestionsItem>,
    dirs: PhantomData<D>
}

impl<D: PMDirs> SuggestionsTable<D> {
    pub fn new() -> Result<Self, SuggestionsError<D::Error>> {
        Ok(Self {
            table: Table::builder(D::new().map_err(SuggestionsError::Dirs)?.suggestions_db())
                .set_read_only()
                .load()?,
            dirs:PhantomData::default(),
        })
    }
    pub fn get_suggestions(&self, path: &Path) -> Vec<&SuggestionsItem> {
        self.table
            .get_table_content()
            .filter_map(|e| {
                for pattern in &e.info.file_types {
                    if glob::glob(path.join(&pattern).to_str()?)
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
    use crate::origins::SuggestionsTable;
    use std::path::{Path, PathBuf};
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
    #[test]
    fn gets_different_sections() {
        let hx = super::get_build_suggestions(&PathBuf::from("tests/mdowns/Helix.md")).unwrap();
        assert_eq!(hx[0].len(), 48);
        let swave =
            super::get_build_suggestions(&PathBuf::from("tests/mdowns/Shortwave.md")).unwrap();
        assert_eq!(swave.len(), 2);
        assert_eq!(swave[0].len(), 10);
        assert_eq!(swave[1].len(), 26);
    }
}
