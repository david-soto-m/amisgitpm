#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod project;
pub use project::{ProjectIface, ProjectStore};
pub mod dirs;
pub use dirs::Directories;
pub mod pm;
pub use pm::{CommonPMErrors, PMInteractive, PMOperations, PMProgrammatic};
