// #![warn(missing_docs)]

//! This is the library associated with the amisgitpm.
//!
//! The idea of this library is to make programmatic interactions with the
//! project as painless as possible.
//! To make everything easy to mix and match there is a preference for
//! trait based interfaces.

pub mod args;
pub mod build_suggestions;
pub mod dirutils;
pub mod interaction;
pub mod package_management;
pub mod projects;
