use crate::{
    interaction::InteractError,
    projects::{Project, ProjectTable},
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
    fn list(&self, prj: &ProjectTable) -> Result<(), InteractError> {
        let mut show_table = pt::Table::new();
        show_table.set_titles(row![
            "Name",
            "Directory name",
            "Project URL",
            "Reference",
            "Update policy"
        ]);
        prj.table.iter().for_each(|(_, e)| {
            show_table.add_row(row![
                e.info.name,
                e.info.dir,
                e.info.url,
                e.info.ref_string,
                e.info.update_policy
            ]);
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
