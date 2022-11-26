use amisgitpm::{
    Interactions, PMBasics, PMDirs, PMExtended, PMInteractive, Project, ProjectStore, UpdatePolicy,
};
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
impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMExtended for ProjectManager<D, PS, I> {
    fn bootstrap(&mut self) -> Result<(), Self::Error> {
        let prj = Project {
            name: "amisgitpm".into(),
            dir: "amisgitpm".into(),
            url: "https://github.com/david-soto-m/amisgitpm.git".into(),
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local/".into()],
            uninstall_script: vec!["cargo uninstall amisgitpm --root ~/.local/".into()],
        };
        self.install(prj)
    }
}
impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMInteractive for ProjectManager<D, PS, I> {
    type Interact = I;

    fn map_inter_error(err: <Self::Interact as Interactions>::Error) -> Self::Error {
        Self::Error::Interact(err)
    }
}
