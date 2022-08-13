use crate::build_suggestions::SuggestionsError;
use markdown_extract;
use regex::Regex;
use std::path::PathBuf;

pub fn get_build_suggestions(readme_file: &PathBuf) -> Result<Vec<Vec<String>>, SuggestionsError> {
    let re = Regex::new(r"((?i)compil|instal|build)").unwrap();
    markdown_extract::extract_from_path(readme_file, &re).map_err(|e| e.into())
}

#[cfg(test)]
mod test {
    //TODO Test extensively
    use std::path::PathBuf;
    #[test]
    fn gets_different_sections() {
        let hx = super::get_build_suggestions(&PathBuf::from("tests/mdowns/Helix.md")).unwrap();
        assert_eq!(hx[0].len(), 48);
        let swave =
            super::get_build_suggestions(&PathBuf::from("tests/mdowns/Shortwave.md")).unwrap();
        assert_eq!(swave.len(), 2);
        assert_eq!(swave[0].len(), 10);
        assert_eq!(swave[1].len(), 26);
    }
}
