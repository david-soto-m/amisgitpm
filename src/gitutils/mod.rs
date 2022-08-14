use crate::{
    build_suggestions::BuildSuggester, dirutils, interaction::InstallInteractions,
    projects::ProjectTable,
};
use git2::Repository;

mod gitutil_error;
pub use gitutil_error::GitUtilError;

pub trait GitUtils {
    type Error: std::error::Error;
    fn interactive_install<T, Q>(url: &str) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions;
}

pub struct GitUtilImpl {}

impl GitUtils for GitUtilImpl {
    type Error = GitUtilError;
    fn interactive_install<T, Q>(url: &str) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions,
    {
        let mut project_table = ProjectTable::new()?;
        let proj_stub = <Q as InstallInteractions>::initial(url, &project_table)
            .map_err(|e| Self::Error::Interact(e.to_string()))?;
        let psite = dirutils::src_dirs().join(&proj_stub.name);

        println!("Starting download, please, wait a bit");

        let repo = Repository::clone(url, &psite)?;
        let ref_name = <Q as InstallInteractions>::refs(&repo)
            .map_err(|e| Self::Error::Interact(e.to_string()))?;
        let (obj, refe) = repo.revparse_ext(&ref_name)?;
        repo.checkout_tree(&obj, None)?;
        match refe {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(obj.id()),
        }?;

        let name = proj_stub.name.to_owned();
        let a = <T as BuildSuggester>::new(&psite)
            .map_err(|e| Self::Error::Suggestions(e.to_string()))?;
        let proj = <Q as InstallInteractions>::finish(proj_stub, a)
            .map_err(|e| Self::Error::Interact(e.to_string()))?;
        project_table.table.push(&name, proj)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
