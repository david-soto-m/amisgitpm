use crate::dbs::{Project, UpdatePolicy};
use dialoguer::{Confirm, Input};

pub mod interact_error;
pub use interact_error::InteractError;

pub fn initial_config(url: &str) -> Result<Project, InteractError> {
    let name = match url.split('/').last() {
        Some(potential_name) => match potential_name.to_string().split('.').next() {
            Some(name) => name.into(),
            None => "".into(),
        },
        None => "".into(),
    };

    let name = if !Confirm::new()
        .with_prompt(format!(
            "Do you want to use the name: {} for the repo?",
            name
        ))
        .interact()?
    {
        loop {
            match Input::<String>::new()
                .with_prompt("Please provide a name")
                .interact_text()
            {
                Ok(name) => break name,
                Err(e) => println!("couldn't quite get it because {e}, please try again"),
            }
        }
    } else {
        name
    };

    Ok(Project {
        name,
        url: url.into(),
        install_script: vec![],
        uninstall_script: vec![],
        branch: "".into(),
        update_policy: UpdatePolicy::Never,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
