use directories::ProjectDirs;

pub const PROJECT_INFO: (&str, &str, &str) = ("org", "amisoft", "amisgitpm");

pub const SRC_DIR: &str = "/src";


pub fn p_dirs() -> ProjectDirs {
    ProjectDirs::from(PROJECT_INFO.0, PROJECT_INFO.1, PROJECT_INFO.2).unwrap()
}

pub fn src_dirs() -> String {
    let mut psite = p_dirs()
        .data_local_dir()
        .to_str()
        .unwrap()
        .to_string();
    psite.push_str(SRC_DIR);
    psite
}
