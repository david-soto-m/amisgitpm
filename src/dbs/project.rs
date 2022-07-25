use crate::dbmanager::{Permissions, Table, TableError};
use serde::{Deserialize, Serialize};

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
        Table::load("db/projects", Permissions::default()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::dbs::Project;
    use std::fs;
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
