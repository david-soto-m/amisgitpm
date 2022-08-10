use crate::suggestions::suggestions_error::SuggestionsError;
use markdown::{self, Block};
use std::fs;
use std::path::Path;
pub fn get_build_suggestions<Q: AsRef<Path>>(readme_file: Q) -> Result<(), SuggestionsError> {
    let token_list = markdown::tokenize(&fs::read_to_string(readme_file)?);
    let mut flag = false;
    let round_one = token_list.iter().filter(|element| {
        match element {
            Block::Header(_, _) => {
                flag = !flag;
                println!("{flag}");
            }
            _ => println!("here"),
        };
        flag
    });
    let code_snippets = token_list.iter().filter(|element| {
        match element {
            Block::CodeBlock(_, _) => {}
            _ => println!("here"),
        }
        false
    });
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn tokenize() {
        super::get_build_suggestions("tests/mdowns/helix.md").unwrap();
    }
}
