use crate::{
    build_suggestions::BuildSuggester,
    dirutils,
    interaction::{InstallInteractions, MinorInteractions, UpdateInteractions},
    projects::Project,
};
mod core;
mod ext;
mod inter;
mod pm_error;

pub trait PackageManagementCore {
    type Error: std::error::Error;
    fn install(pkg_name: &str, prj: &Project) -> Result<(), Self::Error>;
    fn update(pkg_name: &str) -> Result<(), Self::Error>;
    fn uninstall(pkg_name: &str) -> Result<(), Self::Error>;
    fn restore(pkg_name: &str) -> Result<(), Self::Error>;
}

pub trait PackageManagementInteractive: PackageManagementCore {
    fn inter_install<T, Q>(url: &str, path: Option<String>) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions;
    fn inter_update<Q: UpdateInteractions>(package: Option<String>) -> Result<(), Self::Error>;
    fn list<Q: MinorInteractions>() -> Result<(), Self::Error>;
    fn edit<Q: MinorInteractions>(package: &str) -> Result<(), Self::Error>;
}

pub trait PackageManagementExt: PackageManagementCore {
    fn reinstall(package: &str) -> Result<(), Self::Error>;
    fn rebuild(package: &str) -> Result<(), Self::Error>;
    fn rename(old_package_name: &str, new_package_name: &str) -> Result<(), Self::Error>;
    fn cleanup() -> Result<(), Self::Error>;
    fn bootstrap() -> Result<(), Self::Error>;
}

pub struct PackageManager {}
