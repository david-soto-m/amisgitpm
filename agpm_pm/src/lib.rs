use agpm_abstract::{Interactions, PMBasics, PMDirs, PMExtended, PMInteractive, ProjectStore};
use std::marker::PhantomData;

mod error;
pub use error::PMError;
mod operations;

pub struct ProjectManager<D: PMDirs, PS: ProjectStore, I: Interactions> {
    dirs: D,
    store: PS,
    inter_data: PhantomData<I>,
}
impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMBasics for ProjectManager<D, PS, I> {}
impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMExtended for ProjectManager<D, PS, I> {}
impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMInteractive for ProjectManager<D, PS, I> {
    type Interact = I;

    fn map_inter_error(err: <Self::Interact as Interactions>::Error) -> Self::Error {
        Self::Error::Interact(err)
    }
}
