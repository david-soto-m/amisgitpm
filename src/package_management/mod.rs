use crate::{
    build_suggestions::BuildSuggester,
    interaction::{InstallInteractions, MinorInteractions},
    projects::Project,
};
mod pm_error;
pub use pm_error::*;
mod pm;
pub use pm::PackageManager;

pub trait PackageManagement {
    type Error: std::error::Error;
    fn interactive_install<T, Q>(url: &str, path: Option<String>) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions;
    fn install(prj: Project) -> Result<(), Self::Error>;
    fn uninstall(package: &str) -> Result<(), Self::Error>;
    fn reinstall(package: &str) -> Result<(), Self::Error>;
    fn rebuild(package: &str) -> Result<(), Self::Error>;
    fn list<Q: MinorInteractions>() -> Result<(), Self::Error>;
    fn edit<Q: MinorInteractions>(package: &str) -> Result<(), Self::Error>;
    fn cleanup() -> Result<(), Self::Error>;
}
