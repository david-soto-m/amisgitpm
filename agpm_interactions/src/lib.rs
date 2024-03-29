#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use agpm_pm::Interactions;
use agpm_project::{Project, UpdatePolicy};
#[cfg(feature = "suggestions")]
use agpm_suggestions::SuggestionsDirs;
use amisgitpm::{Directories, ProjectStore};
use console::{style, Term};
use dialoguer::{Confirm, Editor, Input, MultiSelect, Select};
use git2::{BranchType, Repository};
use prettytable as pt;
use prettytable::row;
use std::{marker::PhantomData, path::Path};
mod error;
pub use error::InteractError;

/// This struct implements the [`agpm_pm::Interactions`] trait. To that purpose
pub struct Interactor<
    #[cfg(not(feature = "suggestions"))] T: Directories,
    #[cfg(feature = "suggestions")] T: Directories + SuggestionsDirs,
> {
    t: Term,
    dirs: PhantomData<T>,
}

impl<
        #[cfg(not(feature = "suggestions"))] T: Directories,
        #[cfg(feature = "suggestions")] T: Directories + SuggestionsDirs,
    > Interactor<T>
{
    fn get_sugg(&self, sug: &Vec<Vec<String>>, info: &str) -> Result<Vec<String>, InteractError> {
        let mut edit_string = String::new();
        self.t.clear_screen()?;
        if !sug.is_empty() {
            println!("{info}");
            let mut choices = sug.iter().map(|a| a[0].clone()).collect::<Vec<String>>();
            choices.push("Stop previews".into());
            loop {
                let idx = Select::new()
                    .items(&choices)
                    .default(choices.len())
                    .with_prompt(format!(
                        "Please select one of these to preview pressing {}",
                        style("space").bold()
                    ))
                    .interact()?;
                if idx == choices.len() - 1 {
                    break;
                }
                println!("{:#?}", sug[idx]);
            }
            choices.pop().unwrap();
            let choices = MultiSelect::new()
                .items(&choices)
                .with_prompt(
                    format!(
"Please select {} the suggestions you'd like to edit, press {} next to all that apply
when you are done press {}", style("all").bold(), style("space").bold(), style("enter").bold()),
                )
                .report(false)
                .interact()?;
            for &i in &choices {
                for string in &sug[i] {
                    if !edit_string.is_empty() {
                        edit_string.push('\n');
                    }
                    edit_string.push_str(string);
                }
            }
        };
        if let Some(final_install) = Editor::new().edit(&edit_string)? {
            Ok(final_install.split('\n').map(|e| e.to_string()).collect())
        } else {
            Ok(vec![])
        }
    }

    fn get_name_or_dir(
        &self,
        sugg: &str,
        prompts: (&str, &str, &str),
        check: impl Fn(&str) -> bool,
    ) -> Result<String, InteractError> {
        self.t.clear_screen()?;
        println!("{}", prompts.0);
        loop {
            let input: Result<String, _> = Input::new()
                .with_initial_text(sugg)
                .with_prompt(prompts.1)
                .interact();
            match input {
                Ok(dir_candidate) => {
                    if check(&dir_candidate) {
                        println!("{}", prompts.2);
                    } else {
                        break Ok(dir_candidate);
                    }
                }
                Err(e) => {
                    println!("Couldn't quite catch it because of:\n {e}\n, please try again");
                }
            }
        }
    }

    fn get_updates(&self) -> Result<UpdatePolicy, InteractError> {
        self.t.clear_screen()?;
        println!("Now we are trying to get an update policy");
        let update_array = vec![UpdatePolicy::Ask, UpdatePolicy::Always, UpdatePolicy::Never];
        let idx = Select::new().items(&update_array).interact()?;
        Ok(update_array[idx])
    }

    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn provide_suggestions(wher: &Path) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
        #[cfg(feature = "suggestions")]
        {
            return agpm_suggestions::get_suggestions::<T>(wher).unwrap();
        }
        // This code is reachable when the feature suggestions is enabled
        (vec![], vec![])
    }
}

impl<
        #[cfg(not(feature = "suggestions"))] T: Directories,
        #[cfg(feature = "suggestions")] T: Directories + SuggestionsDirs,
        ST: ProjectStore<Project>,
    > Interactions<Project, ST> for Interactor<T>
{
    type Error = InteractError;
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            t: Term::stdout(),
            dirs: PhantomData::default(),
        })
    }
    fn repo_to_checkout_branch(
        &self,
        mut prj: Project,
        repo: &Repository,
    ) -> Result<Project, Self::Error> {
        let branch_arr: Vec<String> = repo
            .branches(Some(BranchType::Local))?
            .filter_map(|br| br.ok())
            .map(|(br, _)| br.into_reference())
            .filter_map(|el| el.name().map(|name| name.to_string()))
            .collect();
        let branch_idx = Select::new()
            .default(0)
            .with_prompt("Please, choose a reference")
            .items(&branch_arr)
            .interact()?;
        prj.ref_string = branch_arr[branch_idx].clone();
        Ok(prj)
    }
    fn create_project(
        &self,
        prj_stub: &Project,
        store: &ST,
        wher: &Path,
    ) -> Result<Project, Self::Error> {
        let (ins, unins) = Self::provide_suggestions(wher);
        let sugg_name = prj_stub
            .url
            .split('/')
            .last()
            .map_or(String::new(), |potential_dir| {
                potential_dir
                    .to_string()
                    .rsplit_once('.')
                    .map_or(potential_dir.to_string(), |(dir, _)| dir.to_string())
            });
        let name = self.get_name_or_dir(
            &sugg_name,
            (
                "What's the name of the project going to be?",
                "Please provide a name for the project",
                "A project already uses that name, please suggest another",
            ),
            |a| !store.check_name_free(a),
        )?;
        let dir = self.get_name_or_dir(
            &name,
            (
                &format!(
                    "What's the {} name of the project going to be?
The directory is a name for a folder",
                    style("directory").bold()
                ),
                "Please provide a directory name",
                "A project already uses that directory, please suggest another",
            ),
            |a| !store.check_dir_free(a),
        )?;
        let update_policy = self.get_updates()?;
        let install_script = self.get_sugg(
            &ins,
            &format!(
                "Now we have to establish how to build and install the program.
Please keep two things in mind:
1) The script will be run from the {} of the project.
2) All the lines in your script will be {}. If you want to detach some
commands you might want to do something like this `command-to-detach & cd .`",
                style("topmost directory").bold(),
                style("joined by `&&`").bold()
            ),
        )?;
        let uninstall_script = self.get_sugg(
            &unins,
            &format!(
                "Now we have to establish how to uninstall the program.
You might want to trace:
- Different executables/binaries
- Cache that the program generates
- Other files you don't think you will want to keep after uninstalling
Please keep two things in mind:
1) The script will be run from the {} of the project.
2) All the lines in your script will be {}. If you want to detach some
commands you might want to do something like this `command-to-detach & cd .`",
                style("topmost directory").bold(),
                style("joined by `&&`").bold()
            ),
        )?;
        self.t.clear_screen()?;
        println!("Setup is finished, starting to build");
        Ok(Project {
            name,
            dir,
            url: prj_stub.url.clone(),
            ref_string: prj_stub.ref_string.clone(),
            update_policy,
            install_script,
            uninstall_script,
        })
    }

    fn edit(&self, prj: Project) -> Result<Project, Self::Error> {
        if let Some(e) = Editor::new().edit(&serde_json::to_string_pretty(&prj)?)? {
            Ok(serde_json::from_str::<Project>(&e)?)
        } else {
            Ok(prj)
        }
    }
    fn list(&self, store: &ST) -> Result<(), Self::Error> {
        let mut show_table = pt::Table::new();
        show_table.set_titles(row![
            "Name",
            "Directory name",
            "Project URL",
            "Reference",
            "Update policy"
        ]);
        if store.is_empty() {
            return Ok(());
        }
        store.iter().for_each(|e| {
            show_table.add_row(row![e.name, e.dir, e.url, e.ref_string, e.update_policy]);
        });
        println!("{show_table}");
        Ok(())
    }
    fn list_one(&self, prj: &Project) -> Result<(), Self::Error> {
        println!("{prj:#?}");
        Ok(())
    }
    fn update_confirm(&self, prj: &Project) -> bool {
        match prj.update_policy {
            UpdatePolicy::Always => true,
            UpdatePolicy::Ask => Confirm::new()
                .with_prompt(format!("Would you like to update {}", prj.name))
                .interact()
                .unwrap_or_default(),
            UpdatePolicy::Never => false,
        }
    }
    fn url_to_download(&self, url: &str) -> Result<Project, Self::Error> {
        let url = if url.ends_with('/') {
            let (a, _) = url.rsplit_once('/').unwrap();
            a
        } else {
            url
        };
        let sugg = url
            .split('/')
            .last()
            .map_or("temp".into(), |potential_dir| {
                potential_dir
                    .to_string()
                    .rsplit_once('.')
                    .map_or(potential_dir.to_string(), |(dir, _)| dir.to_string())
            });

        Ok(Project {
            url: url.to_string(),
            dir: sugg,
            ..Default::default()
        })
    }
}
