use crate::ProjectManager;
use agpm_abstract::*;

impl<D: PMDirs, PS: ProjectStore, I: Interactions> PMBasics for ProjectManager<D, PS, I> {}
