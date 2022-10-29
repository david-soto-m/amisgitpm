use std::path::PathBuf;
/// A trait to standarize the directories that are used in the crate
pub trait PMDirs
where
    Self: Sized,
{
    type Error: std::error::Error;
    /// new the object that implements this trait
    fn new() -> Result<Self, Self::Error>;
    /// where to look path for a projects db.
    fn projects_db(&self) -> PathBuf;
    /// Where to store all the projects code that is going to be built
    fn src(&self) -> PathBuf;
    /// Where to do the git operations. It is separated because this way if git
    /// operations fail you can still have a buildable program
    fn git(&self) -> PathBuf;
    /// Where to store old copies of the projects
    fn old(&self) -> PathBuf;
}
