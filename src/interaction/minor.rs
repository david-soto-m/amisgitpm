use crate::{
    interaction::InteractError,
    projects::{Project, ProjectStore},
};
use dialoguer::{Confirm, Editor};
use prettytable as pt;
use prettytable::row;
use serde_json;

pub trait MinorInteractions {
    fn edit(&self, prj: Project) -> Result<Project, InteractError> {
        if let Some(e) = Editor::new().edit(&serde_json::to_string_pretty(&prj)?)? {
            Ok(serde_json::from_str::<Project>(&e)?)
        } else {
            Ok(prj)
        }
    }
    fn list<T: ProjectStore>(&self, store: &T) -> Result<(), InteractError> {
        let mut show_table = pt::Table::new();
        show_table.set_titles(row![
            "Name",
            "Directory name",
            "Project URL",
            "Reference",
            "Update policy"
        ]);
        store.iter().for_each(|e| {
            show_table.add_row(row![e.name, e.dir, e.url, e.ref_string, e.update_policy]);
        });
        println!("{show_table}");
        Ok(())
    }
    fn list_one(&self, pkg_name: &str, prj: &Project) -> Result<(), InteractError> {
        println!("Name: {pkg_name}");
        println!("{:#?}", prj);
        Ok(())
    }
    fn update_confirm(&self, package_name: &str) -> Result<bool, InteractError> {
        let res = Confirm::new()
            .with_prompt(format!("Would you like to update {}", package_name))
            .interact()?;
        Ok(res)
    }
}
