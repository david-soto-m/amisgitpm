// #![warn(missing_docs)]

pub mod args;
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
