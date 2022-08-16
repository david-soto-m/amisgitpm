use crate::dirutils;
use json_tables::{Deserialize, Serialize, Table, TableError};
use rayon::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum UpdatePolicy {
    /// Update the project to the newest version every time
    Always,
    /// Ask whether to update or not
    Ask,
    /// Do not update the repo
    #[default]
    Never,
}

impl std::fmt::Display for UpdatePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => {
                write!(f, "Always try to update the project")
            }
            Self::Ask => {
                write!(f, "Ask wether ot update or not")
            }
            Self::Never => {
                write!(f, "Never try to update the project")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Project {
    pub name: String,
    pub url: String,
    pub ref_string: String,
    pub update_policy: UpdatePolicy,
    pub install_script: Vec<String>,
    pub uninstall_script: Vec<String>,
}

pub struct ProjectTable {
    pub table: Table<Project>,
}

impl ProjectTable {
    pub fn load() -> Result<Self, TableError> {
        Ok(Self {
            table: Table::builder(dirutils::projects_db()).load()?,
        })
    }
    pub fn check_if_used_name(&self, name: &str) -> bool {
        self.table
            .get_table_keys()
            .par_bridge()
            .any(|p_name| p_name == name)
    }
}

impl std::fmt::Display for ProjectTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use prettytable as pt;
        use prettytable::row;
        let mut table = pt::Table::new();
        self.table.get_table_content().for_each(|e| {
            table.add_row(row![
                e.info.name,
                e.info.url,
                e.info.ref_string,
                e.info.update_policy
            ]);
        });
        write!(f, "{table}")
    }
}
