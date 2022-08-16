use crate::{
    build_suggestions::BuildSuggester,
    interaction::{InstallInteractions, MinorInteractions},
    projects::Project,
};

mod pm_error;
pub use pm_error::{CommonError, EditError, InstallError, ListError, PMError, UninstallError, CleanupError};
mod pm;
pub use pm::PackageManager;

pub trait PackageManagement {
    type Error: std::error::Error;
    fn interactive_install<T, Q>(url: &str) -> Result<(), Self::Error>
    where
        T: BuildSuggester,
        Q: InstallInteractions;
    fn install(prj: Project) -> Result<(), Self::Error>;
    fn uninstall(package: &str) -> Result<(), Self::Error>;
    fn list<Q: MinorInteractions>() -> Result<(), Self::Error>;
    fn edit<Q: MinorInteractions>(package: &str) -> Result<(), Self::Error>;
    fn cleanup() -> Result<(), Self::Error>;
}
