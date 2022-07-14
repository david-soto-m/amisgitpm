use crate::dbmanager::{Permissions, Table};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::ReadDir;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildAux {
    pub file_types: Vec<String>,
    pub build_suggestions: Vec<Vec<String>>,
}

impl Table<BuildAux> {
    pub async fn get_table() -> Table<BuildAux> {
        Table::new("db/build_aux", Permissions::Read).await
    }
    pub fn get_suggestions(&self, files: ReadDir) -> Vec<&BuildAux> {
        files
            .par_bridge()
            .filter_map(|file| {
                let f = file.unwrap_or_else(|error| panic!("{error}"));
                if f.file_type()
                    .unwrap_or_else(|error| panic!("Couldn't establish file type\n{error}"))
                    .is_file()
                {
                    self.elements.par_iter().find_any(|item| {
                        item.file_types.par_iter().any(|suggestion| {
                            *suggestion
                                == f.file_name()
                                    .into_string()
                                    .expect("funky stuff with file names")
                        })
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::dbmanager::{BuildAux, Permissions, Table};
    use ::std::fs;
    #[tokio::test]
    async fn makes_suggestions() {
        let table: Table<BuildAux> = Table::new("tests/db/three_elems", Permissions::Read).await;
        assert_eq!(
            table
                .get_suggestions(fs::read_dir("tests/projects/meson_project").unwrap())
                .len(),
            2
        );
    }
}
