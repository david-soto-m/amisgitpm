pub const ALREADY_EXISTING: &str = "A project with that name or directory already exists";
pub const COPY: &str = "Had an error while copying the project";
pub const EXEC: (&str, &str, &str) = (
    "The",
    "process failed.
Edit the project config with `amisgitpm edit`",
    "
Then rebuild with `amisgitpm rebuild",
);
pub const GIT: (&str, &str) = (
    "Had an error on a Git operation with error:",
    "
Use the command `amisgitpm cleanup` and try again.",
);
pub const MOVE: &str = "Had an error while moving files:";
pub const NON_EXISTANT: (&str, &str) = (
    "You tried to",
    "a project that doesn't exist
To list all projects use `amisgitpm list`",
);
pub const OS_2_STR: &str = "Couldn't convert from &Osstr to utf-8 &str";
pub const READ: &str = "Failed to read a file:";
pub const PATH: &str = "Had an error while accessing the source's path:";
pub const REMOVE: &str = "Had an error removing a file:";
pub const SPAWN: (&str, &str) = (
    "Had an error while spawning the install process:",
    "
Try rebuilding the project with `amisgitpm rebuild {{project_name}}`",
);
pub const TABLE: &str = "Had an error loading a table:";
