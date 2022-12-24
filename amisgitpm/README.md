# The Traits

This project only provides traits. The idea behind this is that a complete
implementation can have pieces from several other implementations and still
work, only having to rewrite parts of it.

A project manager executable that implements all the traits can do six things:

1. Install from an URL
2. Know what has been installed
3. Update installed projects
4. Edit project setups
5. Go back to the previous version
6. Uninstall projects

It divides the implementations in three parts

## The Package Manager

It is composed by tree traits and an error enum

The first trait is the `PMOperations`. It handles the "low level" stuff, so
that the other traits have really easy implementations

The `PMProgramatic` and the `PMInteractive` define methods to achive the six
tasks in different ways. `PMProgramatic` is oriented towards being an API for
programmers interacting with the project manager, and `PMInteractive` is
oriented towards being used by a CLI application or some other way to interact
with final users.

`agpm_pm::ProjectManager` is a struct that implements all three traits

## The Directories

It is the `PMDirs` trait

The directories are where the store should write its stuff and where the project
manager should put the projects.

The project manager uses three different directories
- The src directory, where projects are built
- The git directory, where projects are downloaded and updated (but never built)
- The old directory, where projects are stored in case going back is needed

A structure that implements this trait is the `agpm_dirs::PMDirsImpl`

## The Store

The store is how the project configurations are stored. It holds two traits
`ProjectT` and `ProjectStore`

The first is a trait that allows the project manager traits to interact with
individual project configurations and setups.

The second trait allows project managers to store new projects, and delete them.

The types that implement these traits are `agpm_project::Project` and
`agpm_store::ProjectStore`


