use crate::interaction::{UpdateError, UpdateInteractions};
use dialoguer::Confirm;

pub type UpdateInterImpl = ();

impl UpdateInteractions for UpdateInterImpl {
    type Error = UpdateError;
    fn confirm(package_name: &str) -> Result<bool, UpdateError> {
        Ok(Confirm::new()
            .with_prompt(format!("Would you like to update {}", package_name))
            .interact()?)
    }
}
