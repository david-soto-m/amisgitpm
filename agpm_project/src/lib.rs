#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use amisgitpm::ProjectIface;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// What to do when updating a project
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Default)]
pub enum UpdatePolicy {
    /// Update the project to the newest version every time
    Always,
    /// Ask whether to update or not
    Ask,
    /// Do not update the repo, **default** value
    #[default]
    Never,
}

impl std::fmt::Display for UpdatePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => {
                write!(f, "Always try to update the project")
            }
            Self::Ask => {
                write!(f, "Ask wether ot update or not")
            }
            Self::Never => {
                write!(f, "Never try to update the project")
            }
        }
    }
}

/// `agpm`'s Project structure. It has one extra field. The `update_policy` stores
/// information that determines behavior when interactively trying to update.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Project {
    /// The name of the projects
    pub name: String,
    /// The name of the directory in which the project is going to be stored
    pub dir: String,
    /// The url from which to git clone the project, it can be a file url
    pub url: String,
    /// A string to identify the branch which you want installed
    pub ref_string: String,
    /// Whether to update, ask or never update the project
    pub update_policy: UpdatePolicy,
    /// How to install the project. The elements are joined with && before execution
    pub install_script: Vec<String>,
    /// How to uninstall the project. The elements are joined with && before execution
    pub uninstall_script: Vec<String>,
}

impl ProjectIface for Project {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_dir(&self) -> &str {
        &self.dir
    }
    fn get_url(&self) -> &str {
        &self.url
    }
    fn get_ref_string(&self) -> &str {
        &self.ref_string
    }
    fn get_install(&self) -> &[String] {
        &self.install_script
    }
    fn get_uninstall(&self) -> &[String] {
        &self.uninstall_script
    }
}
