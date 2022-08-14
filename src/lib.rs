// #![warn(missing_docs)]

//! This is the library associated with the amisgitpm.
//!
//! The idea of this library is to make programatic interactions with the
//! project as painless as posible.
//! To make everything easy to mix and match there is a preference for
//! trait based interfaces.

pub mod args;

/// This module defines the trait that enables getting build suggestions.
///
/// It also defines a struct that implements the trait and the auxiliary
/// structs and functions that are needed for that.
pub mod build_suggestions;
pub mod dirutils;
pub mod gitutils;
pub mod interaction;
pub mod projects;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
