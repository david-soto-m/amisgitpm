mod pm_core;
pub use pm_core::{PackageManagementCore, ScriptType};
mod pm_ext;
pub use pm_ext::PackageManagementExt;
mod pm_inter;
pub use pm_inter::PackageManagementInteractive;

mod err_consts;
mod pm_core_err;
mod pm_ext_err;
mod pm_inter_err;

pub mod pm_error {
    pub use super::err_consts::*;
    pub use super::pm_core_err::*;
    pub use super::pm_ext_err::*;
    pub use super::pm_inter_err::*;
}

pub struct PackageManager {}

impl PackageManagementCore for PackageManager {}
impl PackageManagementExt for PackageManager {}
impl PackageManagementInteractive for PackageManager {}
