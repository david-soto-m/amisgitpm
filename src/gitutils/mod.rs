use crate::{interaction::InstallInteractions, projects::ProjectTable, utils};
use git2::Repository;

mod gitutil_error;
pub use gitutil_error::GitUtilError;

pub fn interactive_install<Q: InstallInteractions>(url: &str) -> Result<(), GitUtilError> {
    let mut project_table = ProjectTable::new()?;
    let proj_stub = <Q as InstallInteractions>::initial(url, &project_table)?;
    let psite = utils::src_dirs().join(&proj_stub.name);

    println!("Starting download, please, wait a bit");

    let repo = Repository::clone(url, psite)?;
    let ref_name = <Q as InstallInteractions>::refs(&repo)?;
    let (obj, refe) = repo.revparse_ext(&ref_name)?;
    repo.checkout_tree(&obj, None)?;
    match refe {
        Some(gref) => repo.set_head(gref.name().unwrap()),
        None => repo.set_head_detached(obj.id()),
    }?;

    let name = proj_stub.name.to_owned();
    let proj = <Q as InstallInteractions>::finish(proj_stub);
    project_table.table.push(&name, proj)?;

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
