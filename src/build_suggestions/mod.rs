use std::path::Path;

mod db_suggestions;
mod mdown;
mod suggestions_error;

pub use db_suggestions::{DBSuggestions, DBSuggestionsTable};
pub use suggestions_error::SuggestionsError;

pub struct BuildSuggestions {
    install: Vec<Vec<String>>,
    uninstall: Vec<Vec<String>>,
}

impl BuildSuggestions {}

impl BuildSuggester for BuildSuggestions {
    fn new(path: &Path) -> Result<Self, SuggestionsError> {
        let dir = std::fs::read_dir(path)?;
        let readme_path = path.join("README.md");
        let readme = mdown::get_build_suggestions(&readme_path).unwrap_or(vec![]);
        let db = DBSuggestionsTable::new()?;
        let db_sug = db.get_suggestions(dir);
        Ok(Self {
            install: db_sug
                .iter()
                .flat_map(|&e| e.install_suggestions.to_owned())
                .chain(readme)
                .collect(),
            uninstall: db_sug
                .iter()
                .flat_map(|e| e.uninstall_suggestions.to_owned())
                .collect(),
        })
    }
    fn get_install(&self) -> &Vec<Vec<String>> {
        &self.install
    }
    fn get_uninstall(&self) -> &Vec<Vec<String>> {
        &self.uninstall
    }
}

pub trait BuildSuggester
where
    Self: Sized,
{
    fn new(path: &Path) -> Result<Self, SuggestionsError>;
    fn get_install(&self) -> &Vec<Vec<String>>;
    fn get_uninstall(&self) -> &Vec<Vec<String>>;
}
