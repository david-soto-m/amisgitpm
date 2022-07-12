use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildAux {
    file_types: Vec<String>,
    build_suggestions: Vec<Vec<String>>,
}
