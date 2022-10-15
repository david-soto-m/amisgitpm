use crate::{dirutils::PMDirsImpl, interaction::Interactor, projects::ProjectTable};

mod pm_core;
pub use pm_core::{PackageManagementCore, ScriptType};
mod pm_ext;
pub use pm_ext::PackageManagementExt;
mod pm_inter;
pub use pm_inter::PackageManagementInteractive;

mod error;
pub use error::{CommonError, PMError};

pub struct PackageManagerDefault {}

impl PackageManagementCore for PackageManagerDefault {
    type Store = ProjectTable;
    type Dirs = PMDirsImpl;
    type Error = PMError;
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
impl PackageManagementExt for PackageManagerDefault {}
impl PackageManagementInteractive for PackageManagerDefault {
    type Interact = Interactor;
    type ErrorI = PMError;
}
