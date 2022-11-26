//! This is the crate to use if you completely buy into my particular
//! implementation of amisgitpm

pub mod prelude {
    pub use amisgitpm::*;
}
pub use agpm_dirs::PMDirsImpl;

pub type Interactor = agpm_interactions::Interactor<PMDirsImpl>;
pub type ProjectTable = agpm_store::ProjectTable<PMDirsImpl>;
pub type ProjectManager = agpm_pm::ProjectManager<PMDirsImpl, ProjectTable, Interactor>;
