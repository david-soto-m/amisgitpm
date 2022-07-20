use crate::dbmanager::{Permissions, Table, TableError};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::ReadDir;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildAux {
    pub file_types: Vec<String>,
    pub install_suggestions: Vec<Vec<String>>,
    pub uninstall_suggestions: Vec<Vec<String>>,
}

impl Table<BuildAux> {
    pub async fn get_table() -> Result<Table<BuildAux>, TableError> {
        Table::new("db/build_aux", Permissions::Read).await
    }
    pub async fn get_suggestions(&self, files: ReadDir) -> Vec<&BuildAux> {
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum UpdatePolicy {
    Overwrite,
    QueryOverwrite,
    New,
    QueryNew,
    Query,
    #[default]
    Never,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Project {
    pub name: String,
    pub url: String,
    pub branch: String,
    pub update_policy: UpdatePolicy,
    pub install_script: Vec<String>,
    pub uninstall_script: Vec<String>,
}

impl Table<Project> {
    pub async fn get_table() -> Result<Table<Project>, TableError> {
        Table::new("db/projects", Permissions::ReadWrite).await
    }
}

#[cfg(test)]
mod tests {
    use crate::dbmanager::{BuildAux, Permissions, Project, Table};
    use ::std::fs;
    #[tokio::test]
    async fn makes_suggestions() {
        let table: Table<BuildAux> = Table::new("db/build_aux", Permissions::Read).await.unwrap();
        assert_eq!(
            table
                .get_suggestions(fs::read_dir("tests/projects/mess_project").unwrap())
                .await
                .len(),
            3
        );
    }
    #[tokio::test]
    async fn all_shipped_json_is_correct() {
        Table::<BuildAux>::new("db/build_aux", Permissions::Read)
            .await
            .unwrap();
        assert!(true)
    }
    #[test]
    fn serialize_projects() {
        let a = serde_json::to_value(Project::default()).unwrap();
        println!("{a}");
    }
    #[test]
    fn deserialize() {
        let _: Project =
            serde_json::from_reader(fs::File::open("tests/db/proj/example_1.json").unwrap())
                .unwrap();
    }
}