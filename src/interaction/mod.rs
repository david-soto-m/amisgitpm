use crate::projects::{Project, ProjectStub, ProjectTable, UpdatePolicy};
use dialoguer::{Confirm, Input, Select};
use git2::Repository;

pub mod interact_error;
pub use interact_error::InteractError;

pub trait InstallInteractions {
    fn initial(url: &str, pr_table: &ProjectTable) -> Result<ProjectStub, InteractError>;
    fn refs(repo: &Repository) -> Result<String, InteractError>;
    fn finish(pr: ProjectStub) -> Project;
}

pub struct UserInstallInteractions();

impl InstallInteractions for UserInstallInteractions {
    fn initial(url: &str, table: &ProjectTable) -> Result<ProjectStub, InteractError> {
        let name = url.split('/').last().map_or("".into(), |potential_name| {
            potential_name
                .to_string()
                .rsplit_once('.')
                .map_or("".into(), |(name, _)| name.to_string())
        });
        let name = if name.is_empty()
            || table.check_if_used_name(&name)
            || !Confirm::new()
                .with_prompt(format!(
                    "Do you want to use {name} as the name of this project"
                ))
                .interact()?
        {
            loop {
                let input: Result<String, _> = Input::new()
                    .with_prompt("Please provide a name for the project")
                    .interact();
                match input {
                    Ok(name_candidate) => {
                        if table.check_if_used_name(&name_candidate) {
                            println!("That name is already in use, please try another")
                        } else {
                            break name_candidate;
                        }
                    }
                    Err(e) => {
                        println!("Couldn't quite catch it because of:\n {e}\n, please try again")
                    }
                }
            }
        } else {
            name
        };
        let update_arr = &[UpdatePolicy::Always, UpdatePolicy::Ask, UpdatePolicy::Never];
        let update_idx = Select::new()
            .with_prompt("Please select an update policy")
            .default(2)
            .items(update_arr)
            .interact()?;
        Ok(ProjectStub {
            name,
            url: url.into(),
            update_policy: update_arr[update_idx],
            ref_string: "".to_string(),
        })
    }

    fn refs(repo: &Repository) -> Result<String, InteractError> {
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
    fn finish(pr: ProjectStub) -> Project {
        pr.into()
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
