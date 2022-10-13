use crate::{dirutils::PMDirsImpl, projects::ProjectTable};

mod pm_core;
pub use pm_core::{PackageManagementCore, ScriptType};
mod pm_ext;
pub use pm_ext::PackageManagementExt;
mod pm_inter;
pub use pm_inter::PackageManagementInteractive;

mod error;
pub use error::PMError;

pub struct PackageManager {}

impl PackageManagementCore for PackageManager {
    type Store = ProjectTable;
    type Dirs = PMDirsImpl;
}
impl PackageManagementExt for PackageManager {}
impl PackageManagementInteractive for PackageManager {}
