use crate::ProjectManager;
use agpm_abstract::*;

impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMInteractive for ProjectManager<D, PS, I> {
    type Interact = I;

    fn map_inter_error(err: <Self::Interact as Interactions>::Error)->Self::Error {
        Self::Error::Interact(err)
    }
    fn list(&self, prj_names: Vec<String>) -> Result<(), Self::Error> {
        let inter = Self::Interact::new().map_err(Self::Error::Interact)?;
        if prj_names.is_empty() {
            inter.list(&self.store).map_err(Self::Error::Interact)?;
        } else {
            prj_names.into_iter().try_for_each(|prj_name| {
                let project = self
                    .store
                    .get_ref(&prj_name)
                    .ok_or(CommonPMErrors::NonExisting)?;
                inter.list_one(project).map_err(Self::Error::Interact)?;
                Ok::<_, Self::Error>(())
            })?;
        }
        Ok(())
    }
}
