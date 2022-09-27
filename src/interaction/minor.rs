use crate::{
    interaction::{MinorError, MinorInteractions},
    projects::{Project, ProjectTable},
};
use dialoguer::Editor;
use serde_json;

pub type MinorInterImpl = ();

impl MinorInteractions for MinorInterImpl {
    type Error = MinorError;
    fn edit(prj: &mut Project) -> Result<(), Self::Error> {
        if let Some(e) = Editor::new().edit(&serde_json::to_string_pretty(prj)?)? {
            *prj = serde_json::from_str::<Project>(&e)?;
        }
        Ok(())
    }
    fn list(prj: &ProjectTable) -> Result<(), Self::Error> {
        println!("{prj}");
        Ok(())
    }
    fn list_one(pkg_name: &str, prj: &Project)->Result<(), Self::Error> {
        println!("Name: {pkg_name}");
        println!("{:#?}", prj);
        Ok(())
    }
}
