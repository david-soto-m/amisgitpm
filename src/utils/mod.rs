use directories::ProjectDirs;
use std::path::PathBuf;

mod consts;
use consts::{NEW_DIR, OLD_DIR, SRC_DIR};
use consts::{PROJECT_CONFIG_DIR, PROJECT_INFO, SUGGESTION_DIR};

pub fn p_dirs() -> ProjectDirs {
    ProjectDirs::from(PROJECT_INFO.0, PROJECT_INFO.1, PROJECT_INFO.2).unwrap()
}

pub fn suggestions_db() -> PathBuf {
    p_dirs().config_dir().join(SUGGESTION_DIR)
}

pub fn projects_db() -> PathBuf {
    p_dirs().config_dir().join(PROJECT_CONFIG_DIR)
}

pub fn src_dirs() -> PathBuf {
    p_dirs().data_local_dir().join(SRC_DIR)
}

pub fn new_src_dirs() -> PathBuf {
    p_dirs().data_local_dir().join(NEW_DIR)
}

pub fn old_src_dirs() -> PathBuf {
    p_dirs().data_local_dir().join(OLD_DIR)
}
