use directories::ProjectDirs;
use std::fs;
const PROJECT_INFO: (&str, &str, &str) = ("org", "amisoft", "amisgitpm");
pub const SUGGESTION_DIR: &str = "suggestions";

fn main() {
    let dirs = ProjectDirs::from(PROJECT_INFO.0, PROJECT_INFO.1, PROJECT_INFO.2).unwrap();
    let sugg_dir = dirs.config_dir().join(SUGGESTION_DIR);
    fs::create_dir_all(&sugg_dir).unwrap();
    fs::read_dir("db/suggestions").unwrap().for_each(|file| {
        let file = file.unwrap();
        println!("{:?}", file.file_name());
        fs::copy(file.path(), sugg_dir.join(file.file_name())).unwrap();
    })
}
