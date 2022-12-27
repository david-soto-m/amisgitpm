#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use amisgitpm::Directories;
use directories::ProjectDirs;
use std::path::PathBuf;
use thiserror::Error;

/// An implementor for the [`Directories`](amisgitpm::Directories) trait
pub struct Dirs {
    p_dirs: ProjectDirs,
}

impl Directories for Dirs {
    type Error = DirError;
    ///
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            p_dirs: ProjectDirs::from("org", "amisoft", "agpm").ok_or(Self::Error::HomeNotFound)?,
        })
    }
    ///`~/.config/amisgitpm/projects` in Linux
    fn projects_db(&self) -> PathBuf {
        self.p_dirs.config_dir().join("projects")
    }
    /// `~/.local/share/amisgitpm/src` in Linux
    fn src(&self) -> PathBuf {
        self.p_dirs.data_local_dir().join("src")
    }

    /// `~/.local/share/amisgitpm/git_ops` in Linux
    fn git(&self) -> PathBuf {
        self.p_dirs.data_local_dir().join("git_ops")
    }
    ///`~/.local/share/amisgitpm/old` in Linux
    fn old(&self) -> PathBuf {
        self.p_dirs.data_local_dir().join("old")
    }
}

impl Dirs {
    /// Reference access to the underlying `ProjectDirs` structure
    pub fn get_pdirs(&self) -> &ProjectDirs {
        &self.p_dirs
    }
}

#[cfg(feature = "suggestions")]
impl agpm_suggestions::SuggestionsDirs for Dirs {
    /// An extra function so that its easy to use with suggestions
    fn suggestions(&self) -> PathBuf {
        self.p_dirs.config_dir().join("suggestions")
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
/// An error type to return from the new instance
pub enum DirError {
    /// An error when no project-based default directories can be found
    #[error("Couldn't find a $HOME or equivalent in your platform")]
    HomeNotFound,
}
