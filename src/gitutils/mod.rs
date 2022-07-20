//! docs
use crate::{interaction, utils};
use git2::Repository;

pub async fn install(url: &str, o_name: Option<String>) {
    let name = match o_name {
        Some(name) => name,
        None => url
            .split("/")
            .last()
            .unwrap()
            .to_string()
            .split(".")
            .next()
            .unwrap()
            .to_string(),
    };
    let mut psite = utils::p_dirs()
        .data_local_dir()
        .to_str()
        .unwrap()
        .to_string();
    psite.push('/');
    psite.push_str(&name);
    match Repository::clone(url, psite) {
        Ok(repo) => {
            let proy = interaction::config_repo(repo);
            proy.branch;
            todo!("Set branch for project");
            //run build script
        }
        Err(e) => {
            println!("{e}")
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
