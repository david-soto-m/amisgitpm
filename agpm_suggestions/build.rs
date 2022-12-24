use agpm_dirs::PMDirsImpl;
use amisgitpm::PMDirs;
use std::fs;

fn main() {
    let dirs = PMDirsImpl::new().unwrap();
    let sugg_dir = dirs.suggestions_dir();
    fs::create_dir_all(&sugg_dir).unwrap();
    for file in fs::read_dir("suggestions").unwrap() {
        let file = file.unwrap();
        fs::copy(file.path(), sugg_dir.join(file.file_name())).unwrap();
    }
}
