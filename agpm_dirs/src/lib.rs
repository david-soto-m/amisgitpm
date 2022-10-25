#![warn(missing_docs)]
//! Shortcuts to the different recurrent paths

use agpm_abstract::PMDirs;
use directories::ProjectDirs;
use std::path::PathBuf;
use thiserror::Error;

/// An implementor for the PMDirs trait, it uses the [directories::ProjectDirs]
/// type internally
pub struct PMDirsImpl {
    p_dirs: ProjectDirs,
}

impl PMDirs for PMDirsImpl {
    type Error = DirError;
    /// Creator for the PMDirsImpl object It will **panic** if there is no valid
    /// `$HOME` path or known equivalent in your platform
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            p_dirs: ProjectDirs::from("org", "amisoft", "amisgitpm").unwrap(),
        })
    }
    ///`~/.config/amisgitpm/suggestions` in Linux
    fn suggestions_db(&self) -> PathBuf {
        self.p_dirs.config_dir().join("suggestions")
    }
    ///`~/.config/amisgitpm/projects` in Linux
    fn projects_db(&self) -> PathBuf {
        self.p_dirs.config_dir().join("projects")
    }
    /// `~/.local/share/amisgitpm/src` in Linux
    fn src_dirs(&self) -> PathBuf {
        self.p_dirs.data_local_dir().join("src")
    }

    /// `~/.local/share/amisgitpm/git_ops` in Linux
    fn git_dirs(&self) -> PathBuf {
        self.p_dirs.data_local_dir().join("git_ops")
    }
    ///`~/.local/share/amisgitpm/old` in Linux
    fn old_dirs(&self) -> PathBuf {
        self.p_dirs.data_local_dir().join("old")
    }
}

/// An error type for the initialization of a PMDirsImpl struct
#[derive(Debug, Error)]
pub enum DirError {}
