#![warn(missing_docs)]

//! This crate define traits for
pub mod project;
pub use project::{ProjectStore, ProjectT};
pub mod dirs;
pub use dirs::PMDirs;
pub mod pm;
pub use pm::{CommonPMErrors, PMInteractive, PMOperations, PMProgrammatic};
