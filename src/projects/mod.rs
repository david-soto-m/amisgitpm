use crate::dirutils;
use json_tables::{Deserialize, Serialize, Table, TableError};
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum UpdatePolicy {
    /// Update the project to the newest version every time
    Always,
    /// Ask whether to update or not
    Ask,
    /// Do not update the repo, **default** value
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
    pub dir: String,
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
    pub fn check_if_used_name(&self, pkg_name: &str) -> bool {
        self.table.get_table_keys().any(|s| s == pkg_name)
    }
    pub fn check_if_used_dir(&self, dir: &str) -> bool {
        self.table
            .get_table_content()
            .any(|p_name| p_name.info.dir == dir)
    }
    pub fn check_if_used_name_dir(&self, pkg_name: &str, dir: &str) -> bool{
        self.table.iter().any(|(name, element)|{
            element.info.dir == dir || name == pkg_name
        })
    }
}

impl std::fmt::Display for ProjectTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use prettytable as pt;
        use prettytable::row;
        let mut show_table = pt::Table::new();
        show_table.set_titles(row![
            "Name",
            "Directory name",
            "Project URL",
            "Reference",
            "Update policy"
        ]);
        self.table.iter().for_each(|(name, e)| {
            show_table.add_row(row![
                name,
                e.info.dir,
                e.info.url,
                e.info.ref_string,
                e.info.update_policy
            ]);
        });
        write!(f, "{show_table}")
    }
}
