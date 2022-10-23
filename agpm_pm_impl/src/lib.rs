use agpm_abstract::*;
use std::marker::PhantomData;

mod error;
pub use error::PMError;
mod base;
mod extended;
mod inter;
mod operations;

pub struct ProjectManager<D: PMDirs, PS: ProjectStore, I: Interactions> {
    dirs: D,
    store: PS,
    inter_data: PhantomData<I>,
}
