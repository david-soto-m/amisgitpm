#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    // use crate::origins::SuggestionsTable;
    // use std::path::{Path, PathBuf};
    // #[test]
    // fn makes_suggestions() {
    //     let table = SuggestionsTable::new().unwrap();
    //     let len = table
    //         .get_suggestions(&Path::new("tests/projects/mess_project"))
    //         .len();
    //
    //     assert_eq!(len, 3);
    // }
    // #[test]
    // fn all_build_aux_json_is_correct() {
    //     SuggestionsTable::new().unwrap();
    //     assert!(true)
    // }
    // #[test]
    // fn gets_different_sections() {
    //     let hx = super::get_build_suggestions(&PathBuf::from("tests/mdowns/Helix.md")).unwrap();
    //     assert_eq!(hx[0].len(), 48);
    //     let swave =
    //         super::get_build_suggestions(&PathBuf::from("tests/mdowns/Shortwave.md")).unwrap();
    //     assert_eq!(swave.len(), 2);
    //     assert_eq!(swave[0].len(), 10);
    //     assert_eq!(swave[1].len(), 26);
    // }
}
