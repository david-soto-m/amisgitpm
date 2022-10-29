use crate::SuggestionsError;
use agpm_abstract::PMDirs;
use json_tables::{Deserialize, Serialize, Table};
use std::{marker::PhantomData, path::Path};
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
    dirs: PhantomData<D>,
}

impl<D: PMDirs> SuggestionsTable<D> {
    pub fn new() -> Result<Self, SuggestionsError<D::Error>> {
        Ok(Self {
            table: Table::builder(
                D::new()
                    .map_err(SuggestionsError::Dirs)?
                    .projects_db()
                    .parent()
                    .ok_or_else(|| SuggestionsError::Other("No parent found".into()))?
                    .join("suggestions"),
            )
            .set_read_only()
            .load()?,
            dirs: PhantomData::default(),
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
