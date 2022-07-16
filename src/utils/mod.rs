use directories::ProjectDirs;

pub const  PROJECT_INFO: (&str, &str, &str) = ("org", "amisoft", "amisgitpm");


pub fn p_dirs ()-> ProjectDirs{
    ProjectDirs::from(PROJECT_INFO.0,PROJECT_INFO.1,PROJECT_INFO.2).unwrap()
}
