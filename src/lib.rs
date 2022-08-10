// #![warn(missing_docs)]

pub mod args;
pub mod gitutils;
pub mod interaction;
pub mod projects;
pub mod suggestions;
pub mod utils;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
