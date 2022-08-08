use crate::{dbs::ProjectTable, interaction, utils};
use git2::Repository;

mod gitutil_error;
pub use gitutil_error::GitUtilError;

pub async fn install(url: &str) -> Result<(), GitUtilError> {
    let mut project_table = ProjectTable::new()?;
    let proj_stub = interaction::initial_config(url, &project_table)?;
    let mut psite = utils::src_dirs();
    psite.push('/');
    psite.push_str(&proj_stub.name);
    println!("Starting download, please, wait a bit");
    match Repository::clone(url, psite) {
        Ok(repo) => {
            let ref_name = interaction::ref_config(&repo)?;
            let (obj, refe) = repo.revparse_ext(&ref_name)?;
            repo.checkout_tree(&obj, None)?;
            match refe {
                Some(gref) => repo.set_head(gref.name().unwrap()),
                None => repo.set_head_detached(obj.id()),
            }?;
            interaction::finish_config(proj_stub);
        }
        Err(e) => {
            println!("Couldn't get the repository due to:\n {e}")
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
