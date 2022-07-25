use rayon::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{
    collections::{
        hash_map::ValuesMut,
        HashMap,
    },
    ffi::OsStr,
    fmt::Debug,
    fs::{self, File},
    io::{prelude::*, SeekFrom},
    marker::Send,
};

mod table_error;
pub use table_error::TableError;

mod aux;
pub use aux::{Permissions, TableElement, WriteType};

#[derive(Debug)]
pub struct Table<T>
where
    T: Serialize + DeserializeOwned + Send + Debug + Clone + Sync,
{
    /// A string instead of a ReadDir, because it's easier to modify and write new files from.
    /// (ReadDir doesnt implement clone or copy so it's just annoying to deal with)
    dir: String,
    content: HashMap<String, TableElement<T>>,
    permissions: Permissions,
    is_modified: bool,
}

impl<T> Table<T>
where
    T: Serialize + DeserializeOwned + Send + Debug + Clone + Sync,
{
    pub async fn load(dir: &str, permissions: Permissions) -> Result<Table<T>, TableError> {
        let files: Vec<Result<(String, File), TableError>> = fs::read_dir(dir)?
            .par_bridge()
            .map(|dir_entry| {
                let path = dir_entry.unwrap().path();
                let jstr = OsStr::new("json");
                if Some(jstr) == path.extension() {
                    // we know it has a name, because it ends in .json
                    let name = path.file_name().unwrap().to_str().unwrap().to_string();
                    let name = name.split('.').next().unwrap().to_string();
                    let file = match permissions {
                        Permissions::ReadOnly => File::open(&path),
                        Permissions::Write(_) => File::options().read(true).write(true).open(&path),
                    };
                    match file {
                        Ok(fi) => Ok((name, fi)),
                        Err(e) => Err(TableError::FileOpError(e)),
                    }
                } else {
                    Err(TableError::JsonError)
                }
            })
            .collect();
        let mut content = HashMap::<String, TableElement<T>>::new();
        for element in files.into_iter() {
            match element {
                Ok((name, file)) => {
                    let info = serde_json::from_reader(&file)?;
                    let file = match permissions {
                        Permissions::ReadOnly => None,
                        Permissions::Write(_) => Some(file),
                    };
                    content.insert(name, TableElement { file, info });
                }
                Err(TableError::JsonError) => {}
                Err(e) => return Err(e),
            };
        }
        Ok(Table {
            permissions,
            dir: dir.into(),
            content,
            is_modified: false,
        })
    }

    pub async fn reload(self) -> Result<Table<T>, TableError> {
        Table::load(&self.dir, self.permissions).await
    }

    /// It appends an element to the table and opens a file "fname.json" with
    /// read write permissions.
    /// It doesn't write back the file, it only opens it.
    /// The open file occurs sync
    pub async fn push(&mut self, info_elem: T, fname: &str) -> Result<(), TableError> {
        match self.permissions {
            Permissions::Write(_) => {}
            Permissions::ReadOnly => return Err(TableError::NoWritePermError),
        };
        let mut f_elem = self.dir.clone();
        f_elem.push('/');
        f_elem.push_str(fname);
        f_elem.push_str(".json");
        let f_elem = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&f_elem)?;
        let element = TableElement {
            file: Some(f_elem),
            info: info_elem,
        };
        self.content.insert(fname.into(), element);
        self.is_modified = true;
        Ok(())
    }

    /// Returns true when a mutable reference has been taken or when some item
    /// has been appended. If after an operation there is a writeback it will
    /// return false.
    ///~~~rust
    ///# use tokio_test as tk;
    ///# use amisgitpm::dbs::BuildAux;
    ///# use amisgitpm::dbmanager::{Table, Permissions};
    /// let mut table =
    ///# tk::block_on({
    ///     Table::<BuildAux>::load("tests/db/append", Permissions::default())//.await.unwrap();
    ///# }).unwrap();
    /// let init_len = table.len();
    /// assert_eq!(table.is_modified(), false);
    /// assert_eq!(table.len(), 3);
    ///# tk::block_on(
    /// table.push(BuildAux::default(), "deff")//.await.unwrap();
    ///# ).unwrap();
    /// assert_eq!(table.is_modified(), true);
    /// assert_eq!(table.len(), 4);
    /// table.write_back().unwrap();
    /// assert_eq!(table.is_modified(), false);
    /// assert_eq!(table.len(), 4);
    /// let mut element = table.get_mut_info_iter().next().unwrap();
    /// element.file_types = vec!["hellow".to_string()];
    /// //drops elements
    /// assert_eq!(table.is_modified(), true);
    /// table.write_back().unwrap();
    /// assert_eq!(table.is_modified(), false);
    ///# std::fs::remove_file("tests/db/append/deff.json");
    ///~~~
    /// Thanks to the borrow checker you can't try check if is something is modified
    /// while a there is a mutable reference around.
    pub fn is_modified(&self) -> bool {
        self.is_modified
    }
    pub fn get_info_iter<'r>(
        &'r self,
    ) -> rayon::iter::Map<
        rayon::collections::hash_map::Iter<String, TableElement<T>>,
        fn((&'r String, &'r TableElement<T>)) -> &'r T,
    > {
        self.content.par_iter().map(|(_, element)| &element.info)
    }

    pub fn get_mut_info_iter<'r>(
        &'r mut self,
    ) -> std::iter::Map<ValuesMut<String, TableElement<T>>, fn(&'r mut TableElement<T>) -> &'r mut T>
    {
        self.is_modified = true;
        self.content.values_mut().map(|element| &mut element.info)
    }

    pub fn write_back(&mut self) -> Result<(), TableError> {
        match self.permissions {
            Permissions::Write(_) => {}
            Permissions::ReadOnly => return Err(TableError::NoWritePermError),
        };
        if self.is_modified() {
            self.is_modified = false;
            for table_element in self.content.values_mut() {
                // all will be some
                let file = &mut table_element.file.as_ref().unwrap();
                file.set_len(0)?;
                file.seek(SeekFrom::Start(0))?;
                serde_json::to_writer_pretty(*file, &table_element.info)?
            }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool{
        self.content.is_empty()
    }
}

impl<T> Drop for Table<T>
where
    T: Serialize + DeserializeOwned + Send + Debug + Clone + Sync,
{
    /// Writes back in case the write back is set to automatic
    /// ## Panics
    /// - When there is problems in the write back
    ///
    fn drop(&mut self) {
        if Permissions::default() == self.permissions {
            self.write_back().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dbmanager::{Permissions, Table, TableError};
    use crate::dbs::BuildAux;
    #[tokio::test]
    async fn err_table_doesnt_exist() {
        match Table::<BuildAux>::load("tests/db/two_elems", Permissions::ReadOnly).await {
            Err(TableError::FileOpError(_)) => assert!(true),
            _ => assert!(false),
        }
    }
    #[tokio::test]
    async fn table_has_no_json_files() {
        let table = Table::<BuildAux>::load("tests/db", Permissions::ReadOnly)
            .await
            .unwrap();
        assert_eq!(table.len(), 0)
    }
    #[tokio::test]
    async fn table_loads_correctly() {
        let table: Table<BuildAux> = Table::load("tests/db/three_elems", Permissions::ReadOnly)
            .await
            .unwrap();
        assert_eq!(table.len(), 3)
    }
    #[tokio::test]
    async fn table_processes_with_mix_of_files() {
        let table = Table::<BuildAux>::load("tests/db/mixed_json", Permissions::ReadOnly)
            .await
            .unwrap();
        assert_eq!(table.len(), 2)
    }
    #[tokio::test]
    async fn table_processes_with_mix_of_files_wb() {
        let table = Table::<BuildAux>::load("tests/db/mixed_json", Permissions::default())
            .await
            .unwrap();
        assert_eq!(table.len(), 2)
    }
}
