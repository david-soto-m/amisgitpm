use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, fs::File, marker::Send};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum WriteType {
    Manual,
    #[default]
    Automatic,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Permissions {
    ReadOnly,
    Write(WriteType),
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions::Write(WriteType::default())
    }
}

#[derive(Debug)]
pub struct TableElement<T>
where
    T: Serialize + DeserializeOwned + Send + Debug + Clone,
{
    pub file: Option<File>,
    pub info: T,
}
