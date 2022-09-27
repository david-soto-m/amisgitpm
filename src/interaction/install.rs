use crate::{
    build_suggestions::BuildSuggester,
    interaction::{InstallError, InstallInteractions},
    projects::{Project, ProjectTable, UpdatePolicy},
};
use dialoguer::{Confirm, Editor, Input, MultiSelect, Select};
use git2::Repository;

pub type InstallInterImpl = ();

impl InstallInteractions for InstallInterImpl {
    type Error = InstallError;
    fn initial(url: &str, table: &ProjectTable) -> Result<(String, Project), Self::Error> {
        let dir = url.split('/').last().map_or("".into(), |potential_dir| {
            potential_dir
                .to_string()
                .rsplit_once('.')
                .map_or("".into(), |(dir, _)| dir.to_string())
        });
        let dir = if dir.is_empty()
            || table.check_if_used_dir(&dir)
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
                        if table.check_if_used_dir(&dir_candidate) {
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
        Ok((
            dir.clone(),
            Project {
                dir,
                url: url.into(),
                update_policy: update_arr[update_idx],
                ..Default::default()
            },
        ))
    }

    fn refs(repo: &Repository) -> Result<String, Self::Error> {
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

    fn finish<T: BuildSuggester>(mut pr: Project, sugg: T) -> Result<Project, Self::Error> {
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
}
