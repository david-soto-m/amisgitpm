use crate::{interaction, utils};
use git2::Repository;

pub async fn install(url: &str) -> Result<(), interaction::InteractError> {
    let proy = interaction::initial_config(url)?;
    let mut psite = utils::p_dirs()
        .data_local_dir()
        .to_str()
        .unwrap()
        .to_string();
    psite.push('/');
    psite.push_str(&proy.name);
    match Repository::clone(url, psite) {
        Ok(_repo) => {
            //             proy.branch;
            todo!("Set branch for project");
            //run build script
        }
        Err(e) => {
            println!("{e}")
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
