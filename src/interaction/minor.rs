use crate::{
    interaction::interact_error::*,
    projects::{Project, ProjectTable},
};
use dialoguer::{Confirm, Editor};
use serde_json;

pub trait MinorInteractions {
    fn edit(&self, prj: &mut Project) -> Result<(), MinorInteractError> {
        if let Some(e) = Editor::new()
            .edit(&serde_json::to_string_pretty(prj)?)
            .map_err(MinorInteractError::File)?
        {
            *prj = serde_json::from_str::<Project>(&e)?;
        }
        Ok(())
    }
    fn list(&self, prj: &ProjectTable) -> Result<(), MinorInteractError> {
        println!("{prj}");
        Ok(())
    }
    fn list_one(&self, pkg_name: &str, prj: &Project) -> Result<(), MinorInteractError> {
        println!("Name: {pkg_name}");
        println!("{:#?}", prj);
        Ok(())
    }
    fn update_confirm(&self, package_name: &str) -> Result<bool, MinorInteractError> {
        Confirm::new()
            .with_prompt(format!("Would you like to update {}", package_name))
            .interact()
            .map_err(|e| MinorInteractError::Confirm(e))
    }
}
