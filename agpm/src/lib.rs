pub use agpm_dirs::PMDirsImpl;
use agpm_interactions::Interactor;
use agpm_pm::PrjManager;
pub use agpm_project::{Project, UpdatePolicy};
use agpm_store::Store;

type Interacts = Interactor<PMDirsImpl>;
pub type ProjectStore = Store<PMDirsImpl, Project>;
pub type ProjectManager = PrjManager<Project, PMDirsImpl, ProjectStore, Interacts>;

pub mod args;

pub mod prelude {
    pub use amisgitpm::{
        PMDirs, PMInteractive, PMOperations, PMProgrammatic, ProjectStore, ProjectT,
    };
}
