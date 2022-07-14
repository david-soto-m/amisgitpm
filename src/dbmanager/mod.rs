use rayon::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::marker::Send;
pub mod actual_tables;
pub use actual_tables::*;

#[derive(Debug)]
pub enum Permissions {
    Read,
    ReadWrite,
}

#[derive(Debug)]
pub struct Table<T> {
    pub name: String,
    pub files: Vec<File>,
    pub elements: Vec<T>,
    pub permissions: Permissions,
}

trait TableExt<T> {
    fn get_elements(&self) -> Vec<T>;
}

impl<T> Table<T>
where
    T: Serialize + DeserializeOwned + Send,
{
    pub async fn new(name: &str, permissions: Permissions) -> Table<T> {
        let files: Vec<File> = fs::read_dir(name)
            .unwrap_or_else(|error| panic!("The table {name} does not exist\n{error}"))
            .par_bridge()
            .filter_map(|item| {
                let path = item
                    .as_ref()
                    .unwrap_or_else(|error| {
                        panic!("Having trouble reading the table {name}\n{error}")
                    })
                    .path();
                let jstr = OsStr::new("json");
                if Some(jstr) == path.extension() {
                    Some(
                        File::open(&path)
                            .unwrap_or_else(|error| panic!("Couldn't open {:?} \n{error}", path)),
                    )
                } else {
                    None
                }
            })
            .collect();
        let elements: Vec<T> = files
            .par_iter()
            .map(|f| serde_json::from_reader(f).expect("Failed at parsing element {f} of table"))
            .collect();
        Table {
            name: name.to_string(),
            permissions,
            files,
            elements,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dbmanager::{BuildAux, Permissions, Table};
    #[tokio::test]
    async fn detects_correctly() {
        let table: Table<BuildAux> = Table::new("tests/db/three_elems", Permissions::Read).await;
        assert_eq!(table.files.len(), 3)
    }

    #[tokio::test]
    #[should_panic(expected = "The table tests/db/two_elems does not exist")]
    async fn panics_table_404() {
        let _: Table<BuildAux> = Table::new("tests/db/two_elems", Permissions::Read).await;
    }
    #[tokio::test]
    async fn no_json() {
        let table: Table<BuildAux> = Table::new("tests/db", Permissions::Read).await;
        assert!(table.files.is_empty())
    }
    #[tokio::test]
    async fn gets_json() {
        let table: Table<BuildAux> = Table::new("tests/db/three_elems", Permissions::Read).await;
        assert_eq!(table.elements.len(), 3)
    }
}
