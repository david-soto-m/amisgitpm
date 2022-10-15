use crate::dirutils::{PMDirs, PMDirsImpl};
use json_tables::{Deserialize, Serialize, Table, TableError};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Project {
    pub name: String,
    pub dir: String,
    pub url: String,
    pub ref_string: String,
    pub update_policy: UpdatePolicy,
    pub install_script: Vec<String>,
    pub uninstall_script: Vec<String>,
}

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

pub trait ProjectStore
where
    Self: Sized,
{
    type Error: std::error::Error;
    fn new() -> Result<Self, Self::Error>;
    fn add(&mut self, prj: Project) -> Result<(), Self::Error>;
    fn remove(&mut self, pkg_name: &str) -> Result<(), Self::Error>;
    fn get_ref<'a>(&'a self, pkg_name: &str) -> Option<&'a Project>;
    fn get_clone(&self, pkg_name: &str) -> Option<Project>;
    fn edit(&mut self, old_pkg_name: &str, new_prj: Project) -> Result<(), Self::Error> {
        self.remove(old_pkg_name)?;
        self.add(new_prj)?;
        Ok(())
    }
    fn check_dir(&self, dir: &str) -> bool;
    fn check_name(&self, pkg_name: &str) -> bool;
    fn check_unique(&self, pkg_name: &str, dir: &str) -> bool;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Project> + 'a>;
}

pub struct ProjectTable {
    table: Table<Project>,
}

mod error;
pub use error::ProjectStoreError;

impl ProjectStore for ProjectTable {
    type Error = ProjectStoreError;
    fn new() -> Result<Self, Self::Error> {
        let dirs = PMDirsImpl::new();
        match Table::builder(dirs.projects_db()).load() {
            Ok(table) => Ok(ProjectTable { table }),
            Err(e) => match e {
                TableError::FileOpError(io_err) => match io_err.kind() {
                    std::io::ErrorKind::NotFound => Ok(ProjectTable {
                        table: Table::builder(dirs.projects_db())
                            .set_auto_write()
                            .build()?,
                    }),
                    _ => Err(TableError::FileOpError(io_err))?,
                },
                _ => Err(e)?,
            },
        }
    }
    fn check_name(&self, pkg_name: &str) -> bool {
        self.table
            .get_table_content()
            .any(|s| s.info.name == pkg_name)
    }
    fn check_dir(&self, dir: &str) -> bool {
        self.table
            .get_table_content()
            .any(|p_name| p_name.info.dir == dir)
    }
    fn check_unique(&self, pkg_name: &str, dir: &str) -> bool {
        self.table
            .iter()
            .any(|(name, element)| element.info.dir == dir || name == pkg_name)
    }
    fn get_ref<'a>(&'a self, pkg_name: &str) -> Option<&'a Project> {
        Some(&self.table.get_element(pkg_name)?.info)
    }
    fn get_clone(&self, prj_name: &str) -> Option<Project> {
        Some(self.table.get_element(prj_name)?.info.clone())
    }
    fn add(&mut self, prj: Project) -> Result<(), Self::Error> {
        let name = prj.name.clone();
        self.table.push(name, prj)?;
        Ok(())
    }
    fn remove(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        self.table.pop(prj_name)?;
        Ok(())
    }
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Project> + 'a> {
        Box::new(self.table.get_table_content().map(|e| &e.info))
    }
}
