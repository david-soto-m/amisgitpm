#![warn(missing_docs)]
//! Shortcuts to the different recurrent paths

use directories::ProjectDirs;
use std::path::PathBuf;

mod consts;
use consts::{NEW_DIR, OLD_DIR, SRC_DIR};
use consts::{PROJECT_CONFIG_DIR, PROJECT_INFO, SUGGESTION_DIR};

/// Get the ProjectDirs object
pub fn p_dirs() -> ProjectDirs {
    ProjectDirs::from(PROJECT_INFO.0, PROJECT_INFO.1, PROJECT_INFO.2).unwrap()
}

///Get the location of the suggestions database directory
///`~/.config/amisgitpm/suggestions` in Linux
pub fn suggestions_db() -> PathBuf {
    p_dirs().config_dir().join(SUGGESTION_DIR)
}
///Get the location of the projects database directory
///`~/.config/amisgitpm/projects` in Linux
pub fn projects_db() -> PathBuf {
    p_dirs().config_dir().join(PROJECT_CONFIG_DIR)
}

///Get the location of the used sources directory
///`~/.local/share/amisgitpm/src` in Linux
pub fn src_dirs() -> PathBuf {
    p_dirs().data_local_dir().join(SRC_DIR)
}

///Get the location of the new sources directory
///`~/.local/share/amisgitpm/new` in Linux
pub fn new_src_dirs() -> PathBuf {
    p_dirs().data_local_dir().join(NEW_DIR)
}

///Get the location of the old sources directory
///`~/.local/share/amisgitpm/old` in Linux
pub fn old_src_dirs() -> PathBuf {
    p_dirs().data_local_dir().join(OLD_DIR)
}
