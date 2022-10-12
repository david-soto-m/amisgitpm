#![warn(missing_docs)]
//! Shortcuts to the different recurrent paths

use directories::ProjectDirs;
use std::path::PathBuf;


/// A trait to standarize the directories that are used in the crate
pub trait PMDirs{
    /// new the object that implements this trait
    fn new() -> Self;
    /// where to look path for a suggestions db.
    fn suggestions_db(&self) -> PathBuf;
    /// where to look path for a projects db.
    fn projects_db(&self) -> PathBuf;
    /// Where to store all the projects code that is going to be built
    fn src_dirs(&self) -> PathBuf;
    /// Where to do the git operations. It is separated because this way if git
    /// operations fail you can still have a buildable program
    fn git_dirs(&self) -> PathBuf;
    /// Where to store old copies of the projects
    fn old_dirs(&self) -> PathBuf;
}

/// An implementor for the PMDirs trait, it uses the [directories::ProjectDirs]
/// type internally
pub struct PMDirsImpl {
    p_dirs: ProjectDirs,
}

impl PMDirs for PMDirsImpl {
    /// Creator for the PMDirsImpl object It will **panic** if there is no valid
    /// `$HOME` path or known equivalent in your platform
    fn new() -> Self {
        Self {
            p_dirs: ProjectDirs::from("org", "amisoft", "amisgitpm").unwrap(),
        }
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
