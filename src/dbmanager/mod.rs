use rayon::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::fs::{self, File};
use std::marker::Send;

#[derive(Debug)]
pub enum Permissions {
    Read,
    ReadWrite,
}

#[derive(Debug)]
pub struct Table<T> {
    name: String,
    items: Vec<File>,
    permissions: Permissions,
}

impl<'de, T> Table<T>
where
    T: Serialize + DeserializeOwned + Send,
{
    /// doc
    pub async fn new(name: &str, permissions: Permissions) -> Table<T> {
        let items: Vec<T> = fs::read_dir(name)
            .expect(format!("The table {name} does not exist").as_str())
            .par_bridge()
            .filter_map(|item| {
                let f = File::open(
                    item.as_ref()
                        .expect(format!("Having trouble reading the table {name}").as_str())
                        .path(),
                )
                .expect(format!("Couldn't open {item:?}").as_str());
                serde_json::from_reader(f).ok()
            })
            .collect();
        Table {
            name: name.to_string(),
            permissions,
            items,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dbmanager::{Table, Permissions};
    use crate::dbs::BuildAux;
    #[tokio::test]
    async fn detects_correctly() {
        let table: Table<BuildAux> = Table::new("db/build_aux", Permissions::ReadWrite).await;
        assert!(!table.items.is_empty())
    }

    #[tokio::test]
    #[should_panic(expected = "The table dab/build_aux does not exist")]
    async fn panics_appropriately() {
        let table: Table<BuildAux> = Table::new("dab/build_aux", Permissions::ReadWrite).await;
        assert!(!table.items.is_empty())
    }
}
