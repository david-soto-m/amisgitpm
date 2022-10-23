pub use crate::{
    build_suggestions::SuggestionsError, dirutils::DirError, interaction::InteractError,
    projects::ProjectStoreError,
};
use thiserror::Error;
use amisgitpm_types_traits::CommonError;
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PMError {
    #[error(
        "Had an error on a Git operation
{0}
If you fix the issue try the command
`amisgitpm rebuild {{project_name}} --from_git`"
    )]
    Git(#[from] git2::Error),
    #[error(
        "Error while spawning the install process:
{0}
Try rebuilding the project with `amisgitpm rebuild {{project_name}}`"
    )]
    Spawn(#[from] subprocess::PopenError),
    #[error(transparent)]
    FileExt(#[from] fs_extra::error::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Commons(#[from] CommonError),
    #[error(transparent)]
    Suggestions(#[from] SuggestionsError),
    #[error(transparent)]
    Interact(#[from] InteractError),
    #[error(transparent)]
    ProjectStore(#[from] ProjectStoreError),
    #[error(transparent)]
    Dirs(#[from] DirError),
}
