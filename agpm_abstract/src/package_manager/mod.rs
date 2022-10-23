mod operations;
pub use operations::*;
mod base;
pub use base::*;
mod extended;
pub use extended::*;
mod inter;
pub use inter::*;
#[derive(Debug)]
pub enum ScriptType {
    IScript,
    UnIScript,
}
