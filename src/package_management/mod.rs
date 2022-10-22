use crate::{dirutils::PMDirsImpl, interaction::Interactor, projects::ProjectTable};

mod pm_core;
pub use pm_core::PackageManagementCore;
mod pm_ext;
pub use pm_ext::PackageManagementExt;
mod pm_inter;
pub use pm_inter::PackageManagementInteractive;
mod pm_base;
pub use pm_base::{PackageManagementBase, ScriptType};
mod error;
pub use error::{CommonError, PMError};

pub struct PackageManagerDefault {}

impl PackageManagementBase for PackageManagerDefault {
    type Dirs = PMDirsImpl;
    type Error = PMError;
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
impl PackageManagementCore for PackageManagerDefault {
    type Store = ProjectTable;
    type ErrorC = PMError;
}
impl PackageManagementExt for PackageManagerDefault {}
impl PackageManagementInteractive for PackageManagerDefault {
    type Interact = Interactor;
    type ErrorI = PMError;
}
