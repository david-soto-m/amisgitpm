use rayon::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{
    ffi::OsStr,
    fmt::Debug,
    fs::{self, File},
    marker::Send,
};

pub mod actual_tables;
pub use actual_tables::*;
pub mod table_error;
pub use table_error::TableError;

#[derive(Debug)]
pub enum Permissions {
    Read,
    ReadWrite,
}

#[derive(Debug)]
pub struct Table<T> {
    dir: String,
    files: Vec<File>,
    elements: Vec<T>,
    permissions: Permissions,
}

trait TableExt<T> {
    fn get_elements(&self) -> Vec<T>;
}

impl<T> Table<T>
where
    T: Serialize + DeserializeOwned + Send + Debug,
{
    pub async fn new(dir: &str, permissions: Permissions) -> Result<Table<T>, TableError> {
        let files: Vec<Result<File, TableError>> = fs::read_dir(dir)?
            .par_bridge()
            .map(|dir_entry| {
                let path = dir_entry.unwrap().path();
                let jstr = OsStr::new("json");
                if Some(jstr) == path.extension() {
                    let file = match permissions {
                        Permissions::Read => File::open(path),
                        Permissions::ReadWrite => File::options().write(true).open(path),
                    };
                    match file {
                        Ok(fi) => Ok(fi),
                        Err(e) => Err(TableError::FileOpError(e)),
                    }
                } else {
                    Err(TableError::JsonError)
                }
            })
            .collect();
        let error = files
            .par_iter()
            .any(|elem| matches!(elem, Err(TableError::FileOpError(_))));
        let files: Vec<File> = if error {
            return Err(files
                .into_par_iter()
                .find_any(|elem| matches!(elem, Err(TableError::FileOpError(_))))
                .unwrap()
                .unwrap_err());
        } else {
            files
                .into_par_iter()
                .filter_map(|file_res| match file_res {
                    Err(_) => None,
                    Ok(file) => Some(file),
                })
                .collect()
        };

        let elements: Vec<Result<T, TableError>> = files
            .par_iter()
            .map(|file| match serde_json::from_reader(file) {
                Ok(el) => Ok(el),
                Err(err) => Err(TableError::SerdeError(err)),
            })
            .collect();
        let error = elements.iter().any(|el| el.is_err());

        let elements: Vec<T> = if error {
            let err: TableError = elements
                .into_par_iter()
                .find_any(|elem| elem.is_err())
                .unwrap()
                .unwrap_err();
            return Err(err);
        } else {
            elements
                .into_par_iter()
                .filter_map(|file_res| match file_res {
                    Err(_) => None,
                    Ok(file) => Some(file),
                })
                .collect()
        };

        Ok(Table {
            dir: dir.to_string(),
            permissions,
            files,
            elements,
        })
    }
    pub async fn append(&mut self, el: T, fname: &str) -> Result<(), TableError> {
        match self.permissions {
            Permissions::ReadWrite => {}
            _ => return Err(TableError::NoWritePermError),
        };
        let mut a = self.dir.clone();
        a.push_str(fname);
        a.push_str(".json");
        let json_file = File::open(&a)?;
        serde_json::to_writer(json_file, &el)?;
        let json_file = File::open(&a)?;
        self.files.push(json_file);

        Err(TableError::NoWritePermError)
    }
}

#[cfg(test)]
mod tests {
    use crate::dbmanager::{BuildAux, Permissions, Table, TableError};
    #[tokio::test]
    async fn detects_correctly() {
        let table: Table<BuildAux> = Table::new("tests/db/three_elems", Permissions::Read)
            .await
            .unwrap();
        assert_eq!(table.files.len(), 3);
    }

    #[tokio::test]
    async fn err_table_404() {
        match Table::<BuildAux>::new("tests/db/two_elems", Permissions::Read).await {
            Err(TableError::FileOpError(_)) => assert!(true),
            _ => assert!(false),
        }
    }
    #[tokio::test]
    async fn no_json() {
        let table: Table<BuildAux> = Table::new("tests/db", Permissions::Read).await.unwrap();
        assert!(table.files.is_empty())
    }
    #[tokio::test]
    async fn gets_json() {
        let table: Table<BuildAux> = Table::new("tests/db/three_elems", Permissions::Read)
            .await
            .unwrap();
        assert_eq!(table.elements.len(), 3)
    }
}
