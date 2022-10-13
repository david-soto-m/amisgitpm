mod install;
mod error;
pub use error::InteractError;
mod minor;

pub use install::InstallInteractions;
pub use minor::MinorInteractions;

use crate::build_suggestions::BuildSuggestions;

pub struct Interactor {}
impl InstallInteractions for Interactor {
    type Suggester = BuildSuggestions;
}
impl MinorInteractions for Interactor {}
