#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use agpm_dirs::Dirs;
use agpm_interactions::Interactor;
use agpm_pm::PrjManager;
pub use agpm_project::{Project, UpdatePolicy};
use agpm_store::Store;

/// The interactor thats used coordinating the `Interactor` and the `Dirs` structures.
pub type Interacts = Interactor<Dirs>;
/// The store thats created using the `Store`, `Project, and the `Dirs` structures
pub type ProjectStore = Store<Dirs, Project>;
/// The manager thats created from the `Project`, the `Dirs` structure and the
/// `ProjectStore` and `Interacts` types
pub type ProjectManager = PrjManager<Project, Dirs, ProjectStore, Interacts>;

pub mod args;

/// A module with all the important traits that will be needed to use the above public types
pub mod prelude {
    pub use amisgitpm::{
        Directories, PMInteractive, PMOperations, PMProgrammatic, ProjectIface, ProjectStore,
    };
}
