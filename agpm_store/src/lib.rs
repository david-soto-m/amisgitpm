#![warn(missing_docs)]
//! A module to regulate the information for installed projects and
//! how installed projects are stored and internally queried.

use amisgitpm::{PMDirs, ProjectStore, ProjectT};
use json_tables::{Deserialize, Serialize, Table, TableError};
use std::marker::PhantomData;

mod error;
pub use error::ProjectStoreError;

/// A struct that implements the [`ProjectStore`](amisgitpm::ProjectStore)
/// trait using a [`json_tables::Table`]
pub struct Store<D: PMDirs, T: ProjectT + Serialize + for<'d> Deserialize<'d>> {
    table: Table<T>,
    dirs: PhantomData<D>,
}

impl<D: PMDirs, T> ProjectStore<T> for Store<D, T>
where
    T: ProjectT + Serialize + for<'d> Deserialize<'d>,
{
    type Error = ProjectStoreError<D::Error>;
    fn new() -> Result<Self, Self::Error> {
        let dirs = <D as PMDirs>::new().map_err(Self::Error::Dirs)?;
        match Table::builder(dirs.projects_db()).load() {
            Ok(table) => Ok(Store {
                table,
                dirs: PhantomData::default(),
            }),
            Err(e) => match e {
                TableError::FileOpError(io_err) => match io_err.kind() {
                    std::io::ErrorKind::NotFound => Ok(Store {
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
            .any(|s| s.info.get_name() == prj_name)
    }
    fn check_dir_free(&self, dir: &str) -> bool {
        !self
            .table
            .get_table_content()
            .any(|p_name| p_name.info.get_dir() == dir)
    }
    fn check_unique(&self, prj_name: &str, dir: &str) -> bool {
        !self
            .table
            .get_table_content()
            .any(|element| element.info.get_dir() == dir || element.info.get_name() == prj_name)
    }
    fn get_ref(&self, prj_name: &str) -> Option<&T> {
        Some(&self.table.get_element(prj_name)?.info)
    }
    fn get_clone(&self, prj_name: &str) -> Option<T> {
        Some(self.table.get_element(prj_name)?.info.clone())
    }
    fn add(&mut self, prj: T) -> Result<(), Self::Error> {
        let name = prj.get_name().to_string();
        self.table.push(&name, prj)?;
        Ok(())
    }
    fn remove(&mut self, prj_name: &str) -> Result<(), Self::Error> {
        self.table.pop(prj_name)?;
        Ok(())
    }
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a> {
        Box::new(self.table.get_table_content().map(|e| &e.info))
    }
    fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}
