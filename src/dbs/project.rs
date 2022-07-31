use json_tables::{Table, TableError, Deserialize, Serialize};


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

pub struct TableProject{
    table: Table<Project>,
}

impl TableProject {
    pub fn new() -> Result<TableProject, TableError> {
        Ok(TableProject{
            table: Table::builder("db/projects").load()?,
        })
    }
}

