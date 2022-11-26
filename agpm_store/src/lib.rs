#![warn(missing_docs)]
//! A module to regulate the information for installed projects and
//! how installed projects are stored and internally queried.

use amisgitpm::{PMDirs, Project, ProjectStore};
use json_tables::{Table, TableError};
mod error;
pub use error::ProjectStoreError;
use std::marker::PhantomData;

/// A struct that implements the [`ProjectStore`](amisgitpm::ProjectStore)
/// trait using a [`json_tables::Table`]
pub struct ProjectTable<D: PMDirs> {
    table: Table<Project>,
    dirs: PhantomData<D>,
}

impl<D: PMDirs> ProjectStore for ProjectTable<D> {
    type Error = ProjectStoreError<D::Error>;
    fn new() -> Result<Self, Self::Error> {
        let dirs = <D as PMDirs>::new().map_err(Self::Error::Dirs)?;
        match Table::builder(dirs.projects_db()).load() {
            Ok(table) => Ok(ProjectTable {
                table,
                dirs: PhantomData::default(),
            }),
            Err(e) => match e {
                TableError::FileOpError(io_err) => match io_err.kind() {
                    std::io::ErrorKind::NotFound => Ok(ProjectTable {
                        table: Table::builder(dirs.projects_db())
                            .set_auto_write()
                            .build()?,
                        dirs: PhantomData::default(),
                    }),
                    _ => Err(TableError::FileOpError(io_err))?,
                },
                _ => Err(e)?,
            },
        }
    }
    fn check_name_free(&self, prj_name: &str) -> bool {
        !self
            .table
            .get_table_content()
            .any(|s| s.info.name == prj_name)
    }
    fn check_dir_free(&self, dir: &str) -> bool {
        !self
            .table
            .get_table_content()
            .any(|p_name| p_name.info.dir == dir)
    }
    fn check_unique(&self, prj_name: &str, dir: &str) -> bool {
        !self
            .table
            .get_table_content()
            .any(|element| element.info.dir == dir || element.info.name == prj_name)
    }
    fn get_ref<'a>(&'a self, prj_name: &str) -> Option<&'a Project> {
        Some(&self.table.get_element(prj_name)?.info)
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
    fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}
