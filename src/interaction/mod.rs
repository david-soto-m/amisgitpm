mod error;
use crate::build_suggestions::BuildSuggestions;
use amisgitpm_types_traits::{Interactions, Project, ProjectStore, Suggester, UpdatePolicy};
use console::{style, Term};
use dialoguer::{Confirm, Editor, Input, MultiSelect, Select};
pub use error::InteractError;
use git2::{BranchType, Repository};
use prettytable as pt;
use prettytable::row;
use serde_json;
use std::path::Path;

/*
"Now we are trying to establish build instructions. To help with that we have
compiled some suggestions. These come from previous knowledge about build
systems or the README.md file. Assume the commands you leave will start
executing in the root directory of the project."
*/
pub struct Interactor {
    t: Term,
}

impl Interactions for Interactor {
    type Suggester = BuildSuggestions;
    type Error = InteractError;
    fn new() -> Result<Self, Self::Error> {
        Ok(Self { t: Term::stdout() })
    }
    fn refs(&self, repo: &Repository) -> Result<String, Self::Error> {
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
        Ok(branch_arr[branch_idx].to_owned())
    }

    fn get_sugg(&self, sug: &Vec<Vec<String>>, info: &str) -> Result<Vec<String>, Self::Error> {
        {
            self.t.clear_screen()?;
            let sug_len = sug.len() as isize;
            let mut idx: isize = sug_len - 1; // if there are no suggestions idx is -1;
            let mut edit_string = String::new();
            if idx >= 0 {
                println!("{}", info);
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
                    sug[i].iter().for_each(|string| {
                        if !edit_string.is_empty() {
                            edit_string.push('\n');
                        }
                        edit_string.push_str(string)
                    })
                });
            };
            if let Some(final_install) = Editor::new().edit(&edit_string)? {
                Ok(final_install.split('\n').map(|e| e.to_string()).collect())
            } else {
                Ok(vec![])
            }
        }
    }

    fn get_name_or_dir(
        &self,
        sugg: &str,
        prompts: (&str, &str, &str),
        check: impl Fn(&str) -> bool,
    ) -> Result<String, Self::Error> {
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

    fn get_updates(&self) -> Result<UpdatePolicy, Self::Error> {
        self.t.clear_screen()?;
        println!("Now we are trying to get an update policy");
        let update_array = vec![UpdatePolicy::Ask, UpdatePolicy::Always, UpdatePolicy::Never];
        let idx = Select::new().items(&update_array).interact()?;
        Ok(update_array[idx])
    }
    fn create_project(
        &self,
        path: &Path,
        prj_stub: &Project,
        store: &impl ProjectStore,
    ) -> Result<Project, Self::Error> {
        let sugg_name = prj_stub
            .url
            .split('/')
            .last()
            .map_or("".into(), |potential_dir| {
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
        let sugg = Self::Suggester::new(path)?;
        let install_script = self.get_sugg(
            sugg.get_install(),
            "Now we have to establish how to build and install the program.
Please keep two things in mind:
1) The script will be run from the topmost directory of the project.
2) All the lines in your script will be joined by `&&`. If you want to detach some
commands you might want to do something like this `command-to-detach & cd .`",
        )?;
        let uninstall_script = self.get_sugg(
            sugg.get_uninstall(),
            "Now we have to establish how to uninstall the program.
You might want to trace:
- Different executables/binaries
- Cache that the program generates
- Other files you don't think you will want to keep after uninstalling
Please keep two things in mind:
1) The script will be run from the topmost directory of the project.
2) All the lines in your script will be joined by `&&`. If you want to detach some
commands you might want to do something like this `command-to-detach & cd .`",
        )?;
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
    fn list<T: ProjectStore>(&self, store: &T) -> Result<(), Self::Error> {
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
