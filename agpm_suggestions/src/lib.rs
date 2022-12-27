#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use amisgitpm::Directories;
use glob::{GlobError, PatternError};
use json_tables::{Deserialize, Serialize, Table, TableBuilderError, TableError};
use regex::Regex;
use reqwest::blocking;
use std::path::{Path, PathBuf};
use thiserror::Error;

const REGISTRY: [&str; 6] = [
    "bash.json",
    "cargo.json",
    "meson.json",
    "cmain.json",
    "cmake.json",
    "makefile.json",
];

/// A trait to get a suggestions directory for the DB
pub trait SuggestionsDirs: Directories {
    /// Get the suggestions directory
    fn suggestions(&self) -> PathBuf;
}

/// A structure that holds the information needed to detect and suggest
/// some build instructions
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct SuggestionsItem {
    /// File names to detect in order to make the suggestions contained bellow
    pub file_types: Vec<String>,
    /// The set of suggestions in order to build and install a project
    pub install_suggestions: Vec<Vec<String>>,
    /// The set of suggestion in order to uninstall a project
    pub uninstall_suggestions: Vec<Vec<String>>,
}

struct SuggestionsTable {
    table: Table<SuggestionsItem>,
}

fn get_build_suggestions(readme_file: &PathBuf) -> Result<Vec<Vec<String>>, SuggestionsError> {
    let regex = Regex::new(r"((?i)compil|instal|build)").unwrap();
    markdown_extract::extract_from_path(readme_file, &regex).map_err(|e| e.into())
}

type InstallSugs = Vec<Vec<String>>;
type UninstallSugs = Vec<Vec<String>>;
type Suggestions = (InstallSugs, UninstallSugs);

/// Get the suggestions for a given path to a project directory
///
/// There all paths conforming to *.md will be examined according to the following regular expresion
/// `r"((?i)compil|instal|build)"` in any of their headings
///
/// They will also be examined for conformities with the any known structures in the suggestions db
/// such as being a cargo project or having a Makefile.
///
/// # Errors
/// - Can't convert a path to a utf8 encoded str
pub fn get_suggestions<P: SuggestionsDirs>(
    for_: impl AsRef<Path>,
) -> Result<Suggestions, SuggestionsError> {
    let from = P::new()
        .map_err(|e| SuggestionsError::DirsError(e.to_string()))?
        .suggestions();

    let mut readme: Vec<Vec<String>> = vec![];
    for each in glob::glob(
        for_.as_ref()
            .join("*.md")
            .to_str()
            .ok_or(SuggestionsError::Path)?,
    )? {
        readme.append(&mut get_build_suggestions(&each?).unwrap_or_default());
    }
    match SuggestionsTable::new(from.as_ref()) {
        Ok(db) => {
            let db_sug = db.get_suggestions(for_.as_ref());
            Ok((
                db_sug
                    .iter()
                    .flat_map(|&e| e.install_suggestions.clone())
                    .chain(readme)
                    .collect(),
                db_sug
                    .iter()
                    .flat_map(|e| e.uninstall_suggestions.clone())
                    .collect(),
            ))
        }
        Err(_) => Ok((readme, vec![])),
    }
}

/// Downloads all elements in the REGISTRY and stores them.
///
/// It either creates a new table at the `SuggestionsDir` or reads from an existing
/// table, and writes into it.
/// # Panics
/// Not really, but it does have an unwrap where if there is a `REGISTRY` file that
/// doesn't have .json it would panic, but there isn't.
pub fn download_resources<P: SuggestionsDirs>() -> Result<(), SuggestionsError> {
    let f_str = "https://raw.githubusercontent.com/david-soto-m/amisgitpm/main/agpm_suggestions/suggestions/";
    let dir = P::new()
        .map_err(|e| SuggestionsError::DirsError(e.to_string()))?
        .suggestions();

    let mut table = match Table::<SuggestionsItem>::builder(&dir)
        .set_auto_write()
        .build()
    {
        Ok(table) => table,
        Err(TableBuilderError::TableAlreadyExistsError) => {
            Table::builder(&dir).set_auto_write().load()?
        }
        Err(a) => return Err(a.into()),
    };
    for file in REGISTRY {
        let (name, _) = file.rsplit_once('.').unwrap(); // Guaranteed by me that this doesn't panic
        let url = format!("{f_str}{file}");
        let item: SuggestionsItem = blocking::get(url)?.json()?;
        match table.get_mut_element(name) {
            Some(el) => el.info = item,
            None => {
                table.push(name, item)?;
            }
        };
    }
    Ok(())
}

impl SuggestionsTable {
    pub fn new(dir: &Path) -> Result<Self, TableError> {
        Ok(Self {
            table: Table::builder(dir).set_read_only().load()?,
        })
    }
    pub fn get_suggestions(&self, path: &Path) -> Vec<&SuggestionsItem> {
        self.table
            .get_table_content()
            .filter_map(|e| {
                for pattern in &e.info.file_types {
                    if glob::glob(path.join(pattern).to_str()?)
                        .ok()?
                        .next()
                        .is_some()
                    {
                        return Some(&e.info);
                    }
                }
                None
            })
            .collect()
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
/// An error type for the `BuildSuggestions` struct.
pub enum SuggestionsError {
    /// The creation has had an error with some file operation
    #[error(transparent)]
    FileOp(#[from] std::io::Error),
    /// The creation has had an error with a json_table
    #[error(transparent)]
    Table(#[from] TableError),
    /// The creation has had an error while creating a json_table
    #[error(transparent)]
    TableBuild(#[from] TableBuilderError),
    /// Couldn't read file to determine if it matches pattern
    #[error(transparent)]
    Glob(#[from] GlobError),
    /// Reqwest error, while downloading sources
    #[error(transparent)]
    Reqw(#[from] reqwest::Error),
    /// A glob pattern was bad
    #[error(transparent)]
    Pattern(#[from] PatternError),
    /// The path is not utf-8
    #[error("A path is not utf-8 compatible")]
    Path,
    /// A field to place errors that don't fit in with the other variants when
    /// re-implementing the BuildSuggestions
    #[error("{0}")]
    DirsError(String),
}

#[cfg(test)]
mod tests {
    use super::SuggestionsTable;
    use std::path::{Path, PathBuf};
    #[test]
    fn makes_suggestions() {
        let db_loc = Path::new("suggestions");
        let table = SuggestionsTable::new(db_loc).unwrap();
        let len = table
            .get_suggestions(&Path::new("../tests/projects/mess_project"))
            .len();
        assert_eq!(len, 3);
    }
    #[test]
    fn all_build_aux_json_is_correct() {
        let db_loc = Path::new("suggestions");
        SuggestionsTable::new(db_loc).unwrap();
    }
    #[test]
    fn gets_different_sections() {
        let hx = super::get_build_suggestions(&PathBuf::from("../tests/mdowns/Helix.md")).unwrap();
        assert_eq!(hx[0].len(), 48);
        let swave =
            super::get_build_suggestions(&PathBuf::from("../tests/mdowns/Shortwave.md")).unwrap();
        assert_eq!(swave.len(), 2);
        assert_eq!(swave[0].len(), 10);
        assert_eq!(swave[1].len(), 26);
    }
}
