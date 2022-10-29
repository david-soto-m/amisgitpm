pub mod prelude{
pub use agpm_abstract::*;
}
pub use agpm_dirs::PMDirsImpl;

pub type Suggestions = agpm_suggestions::Suggestions<PMDirsImpl>;
pub type Interactor = agpm_interactions::Interactor<Suggestions>;
pub type ProjectTable = agpm_store::ProjectTable<PMDirsImpl>;
pub type ProjectManager = agpm_pm::ProjectManager<PMDirsImpl, ProjectTable, Interactor>;
