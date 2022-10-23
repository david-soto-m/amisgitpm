use crate::*;
pub trait PMInteractive: PMBasics {
    type Interact: Interactions;
    fn inter_install(&mut self, url: &str) -> Result<(), Self::Error>;
    fn list(&self, prj_names: Vec<String>) -> Result<(), Self::Error>;
    fn inter_edit(&mut self, package: &str) -> Result<(), Self::Error>;
    fn inter_update(&self, prj_name: Option<String>, force: bool) -> Result<(), Self::Error>;
}
