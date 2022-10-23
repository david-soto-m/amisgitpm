use crate::*;

pub trait PMExtended: PMBasics {
    fn reinstall(&mut self, prj_name: &str) -> Result<(), Self::Error>;
    fn rebuild(&self, prj_name: &str) -> Result<(), Self::Error>;
    fn bootstrap(&mut self) -> Result<(), Self::Error>;
}
