use crate::build_suggestions::SuggestionsError;
use markdown_extract;
use regex::Regex;
use std::path::PathBuf;

pub fn get_build_suggestions(readme_file: &PathBuf) -> Result<(), SuggestionsError> {
    let re = Regex::new(r"((?i)compil|instal|build)").unwrap();
    let round_one = markdown_extract::extract_from_path(&readme_file, &re)?;
    Ok(())
}

#[cfg(test)]
mod test {
    //TODO Test extensibly
    use std::path::PathBuf;
    #[test]
    fn tokenize() {
        super::get_build_suggestions(&PathBuf::from("tests/mdowns/Helix.md")).unwrap();
        super::get_build_suggestions(&PathBuf::from("tests/mdowns/Shortwave.md")).unwrap();
    }
}
