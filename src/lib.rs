// #![warn(missing_docs)]

pub mod args;
pub mod dbs;
pub mod gitutils;
pub mod interaction;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
