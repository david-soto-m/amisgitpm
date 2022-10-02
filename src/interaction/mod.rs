mod install;
pub mod interact_error;
mod minor;

pub use install::InstallInteractions;
pub use minor::MinorInteractions;

pub struct Interactor {}
impl InstallInteractions for Interactor {}
impl MinorInteractions for Interactor {}
