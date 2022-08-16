use crate::{
    dirutils,
    interaction::{MinorError, MinorInteractions},
    projects::{Project, ProjectTable},
};
use dialoguer::Editor;
use serde_json;

pub type MinorInteractionsImpl = ();

impl MinorInteractions for MinorInteractionsImpl {
    type Error = MinorError;
    fn edit(prj: &mut Project) -> Result<(), Self::Error> {
        let path = dirutils::projects_db().join(format!("{}.json", &prj.name));
        if let Some(e) = Editor::new().edit(&std::fs::read_to_string(path)?)? {
            *prj = serde_json::from_str::<Project>(&e)?;
        }
        Ok(())
    }
    fn list(prj: &ProjectTable) -> Result<(), Self::Error> {
        println!("{prj}");
        Ok(())
    }
}
