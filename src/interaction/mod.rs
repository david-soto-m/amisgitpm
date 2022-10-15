mod error;
use crate::{
    build_suggestions::{BuildSuggester, BuildSuggestions},
    dirutils::{PMDirs, PMDirsImpl},
    projects::{Project, ProjectStore, UpdatePolicy},
};
use dialoguer::{Confirm, Editor, Input, MultiSelect, Select};
pub use error::InteractError;
use git2::Repository;
use prettytable as pt;
use prettytable::row;
use serde_json;

pub trait Interactions
where
    Self: Sized,
{
    type Suggester: BuildSuggester;
    type Error: std::error::Error
        + From<git2::Error>
        + From<serde_json::Error>
        + From<std::io::Error>
        + From<<Self::Suggester as BuildSuggester>::Error>;
    fn new() -> Result<Self, Self::Error>;
    fn initial<T: ProjectStore>(&self, url: &str, table: &T) -> Result<Project, Self::Error> {
        let dir = url.split('/').last().map_or("".into(), |potential_dir| {
            potential_dir
                .to_string()
                .rsplit_once('.')
                .map_or("".into(), |(dir, _)| dir.to_string())
        });
        let dir = if dir.is_empty()
            || table.check_dir(&dir)
            || !Confirm::new()
                .with_prompt(format!(
                    "Do you want to use {dir} as the directory name of this project"
                ))
                .interact()?
        {
            loop {
                let input: Result<String, _> = Input::new()
                    .with_prompt("Please provide a directory name for the project")
                    .interact();
                match input {
                    Ok(dir_candidate) => {
                        if table.check_dir(&dir_candidate) {
                            println!("That name is already in use, please try another")
                        } else {
                            break dir_candidate;
                        }
                    }
                    Err(e) => {
                        println!("Couldn't quite catch it because of:\n {e}\n, please try again")
                    }
                }
            }
        } else {
            dir
        };
        let update_arr = &[UpdatePolicy::Always, UpdatePolicy::Ask, UpdatePolicy::Never];
        let update_idx = Select::new()
            .with_prompt("Please select an update policy")
            .default(2)
            .items(update_arr)
            .interact()?;
        println!("The download will start shortly, please wait");
        Ok(Project {
            name: dir.clone(),
            dir,
            url: url.into(),
            update_policy: update_arr[update_idx],
            ..Default::default()
        })
    }

    fn refs(&self, repo: &Repository) -> Result<String, Self::Error> {
        let branch_arr: Vec<String> = repo
            .references()?
            .filter_map(|res| res.ok())
            .filter_map(|el| el.name().map(|name| name.to_string()))
            .collect();
        let branch_idx = Select::new()
            .default(0)
            .with_prompt("Please, choose a reference")
            .items(&branch_arr)
            .interact()?;
        Ok(branch_arr[branch_idx].to_owned())
    }

    fn finish(&self, mut pr: Project) -> Result<Project, Self::Error> {
        let suggestions_dir = PMDirsImpl::new().src_dirs().join(&pr.dir);
        let sugg = Self::Suggester::new(&suggestions_dir)?;

        {
            let sug = sugg.get_install();
            let sug_len = sug.len() as isize;
            let mut idx: isize = sug_len - 1; // if there are no suggestions idx is -1;
            let mut edit_string = String::new();
            if idx >= 0 {
                println!(
                    "Now we are trying to establish build instructions. To help with that we have
compiled some suggestions. These come from previous knowledge about build
systems or the README.md file. Assume the commands you leave will start
executing in the root directory of the project."
                );
                let mut choices = sug.iter().map(|a| a[0].clone()).collect::<Vec<String>>();

                choices.push("Stop previews".into());
                while idx != sug_len {
                    idx = Select::new()
                        .items(&choices)
                        .with_prompt("Please select one of these to preview")
                        .default(sug_len as usize)
                        .interact()? as isize;
                    if idx != sug_len {
                        println!("{:#?}", sug[idx as usize]);
                    }
                }
                choices.pop().unwrap();
                let choices = MultiSelect::new()
                    .items(&choices)
                    .with_prompt(
"Please select all the suggestions you'd like to edit, press space next to all that apply"
                    )
                    .report(false)
                    .interact()?;
                choices.iter().for_each(|&i| {
                    sug[i].iter().for_each(|string| {
                        if !edit_string.is_empty() {
                            edit_string.push('\n');
                        }
                        edit_string.push_str(string)
                    })
                });
            } else {
                println!("There were no suggestions, please provide a build script")
            }
            if let Some(final_install) = Editor::new().edit(&edit_string)? {
                pr.install_script = final_install.split('\n').map(|e| e.to_string()).collect()
            }
        }
        {
            let sug = sugg.get_uninstall();
            let sug_len = sug.len() as isize;
            let mut idx: isize = sug_len - 1; // if there are no suggestions idx is -1;
            let mut edit_string = String::new();
            if idx >= 0 {
                println!("Now we are doing the same for the uninstall process");
                let mut choices = sug.iter().map(|a| a[0].clone()).collect::<Vec<String>>();
                choices.push("Stop previews".into());
                while idx != sug_len {
                    idx = Select::new()
                        .items(&choices)
                        .default(sug_len as usize)
                        .with_prompt("Please select one of these to preview")
                        .interact()? as isize;
                    if idx != sug_len {
                        println!("{:#?}", sug[idx as usize]);
                    }
                }
                choices.pop().unwrap();
                let choices = MultiSelect::new()
                    .items(&choices)
                    .with_prompt(
    "Please select all the suggestions you'd like to edit, press space next to all that apply"
                    )
                    .report(false)
                    .interact()?;
                choices.iter().for_each(|&i| {
                    sug[i]
                        .iter()
                        .for_each(|string| edit_string.push_str(string))
                });
            }
            if let Some(final_install) = Editor::new().edit(&edit_string)? {
                pr.uninstall_script = final_install.split('\n').map(|e| e.to_string()).collect()
            }
        }
        Ok(pr)
    }

    fn edit(&self, prj: Project) -> Result<Project,Self::Error> {
        if let Some(e) = Editor::new().edit(&serde_json::to_string_pretty(&prj)?)? {
            Ok(serde_json::from_str::<Project>(&e)?)
        } else {
            Ok(prj)
        }
    }
    fn list<T: ProjectStore>(&self, store: &T) -> Result<(), Self::Error> {
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
    fn list_one(&self, pkg_name: &str, prj: &Project) -> Result<(), Self::Error> {
        println!("Name: {pkg_name}");
        println!("{:#?}", prj);
        Ok(())
    }
    fn update_confirm(&self, package_name: &str) -> Result<bool, Self::Error> {
        let res = Confirm::new()
            .with_prompt(format!("Would you like to update {}", package_name))
            .interact()?;
        Ok(res)
    }
}

pub struct Interactor {}
impl Interactions for Interactor {
    type Suggester = BuildSuggestions;
    type Error = InteractError;
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
