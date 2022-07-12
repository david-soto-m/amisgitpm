// #![warn(missing_docs)]

// pub mod dbmanager;
pub mod args;
pub mod dbs;
pub mod gitutils;
pub mod interaction;
pub mod utils;

pub fn silly() {
    sillier();
}

fn sillier() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
