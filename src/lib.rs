#![warn(missing_docs)]

//! docs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

///
pub fn silly() {
    sillier()
}

fn sillier() {}
