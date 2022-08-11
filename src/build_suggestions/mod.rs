use std::path::Path;

mod db_suggestions;
mod mdown;
mod suggestions_error;

pub use db_suggestions::{DBSuggestions, DBSuggestionsTable};
pub use suggestions_error::SuggestionsError;

pub struct BuildSuggestions {}

impl BuildSuggestions {}

impl BuildSuggester for BuildSuggestions {
    fn new<Q: AsRef<Path>>(_path: Q) -> Self {
        Self {}
    }
    fn get_install(&self) -> Vec<Vec<String>> {
        vec![]
    }
    fn get_uninstall(&self) -> Vec<Vec<String>> {
        vec![]
    }
}

pub trait BuildSuggester {
    fn new<Q: AsRef<Path>>(path: Q) -> Self;
    fn get_install(&self) -> Vec<Vec<String>>;
    fn get_uninstall(&self) -> Vec<Vec<String>>;
}
