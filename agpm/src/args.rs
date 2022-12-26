//!This module deals with the arguments for amisgitpm.
//!
//!It uses the clap crate to deal with them. It takes sub-commands
//!facilitated by the enum `Command`.
//!This means that you interact with the CLI of the program is like this:
//!```bash
//!$ amisgitpm install https://github.com/helix-editor/helix
//!```
use clap::{Parser, Subcommand};
#[derive(Parser, Debug, Clone)]
#[clap(version, about)]
/// This struct is the one that takes the arguments from the command line.
pub struct Cli {
    #[clap(subcommand)]
    /// This argument holds an enum with the different subcommands
    pub com: Commands,
}

#[derive(Subcommand, Debug, Clone)]
/// This are the possible commands
pub enum Commands {
    /// Install a new git repo. It installs from URLs of two kinds.
    ///
    /// 1. git clone URL eg. `https://github.com/helix-editor/helix.git`
    ///
    /// 2. Local git repo URL. `///file:///home/user/Path/to/git/project`
    Install {
        #[clap(value_parser)]
        /// the git repo url
        /// To install a remote repo
        ///```bash
        ///
        ///amisgitpm install https://github.com/helix-editor/helix.git
        ///
        ///```
        /// To install a local repo
        ///
        ///```bash
        ///
        ///amisgitpm install file:///home/user/Path/to/git/project
        ///
        ///```
        url: String,
    },

    /// Update project(s)
    Update {
        #[clap(value_parser)]
        /// An optional list of project names to update independently.
        ///
        /// If not provided all projects are updated
        project: Vec<String>,
    },

    /// Update the suggestions, downloading all of them, and substituting those
    /// already present
    UpdateSuggestions,

    /// Uninstall a project
    Uninstall {
        #[clap(value_parser)]
        /// The project name to uninstall
        project: Vec<String>,
    },

    /// Get the last version of the project
    Restore {
        #[clap(value_parser)]
        /// The project to downgrade
        project: Vec<String>,
    },

    /// Uninstall then install a project
    Reinstall {
        #[clap(value_parser)]
        /// The project name to reinstall
        project: String,
    },

    /// Run the build instructions of a project
    Rebuild {
        #[clap(value_parser)]
        /// The project name to rebuild
        project: String,
    },

    /// Remove all srcs with no project associated
    ///
    /// It is `O(N^2)`, with `N` the number of installed projects
    /// It is parallelized, and therefore is panicky rather than reporting
    Clean,

    /// Edit the configuration of a project
    Edit {
        #[clap(value_parser)]
        /// The name of project to edit
        project: String,
    },

    /// Show the list of installed applications and their version
    List {
        #[clap(value_parser)]
        /// Packages from which to get detailed information
        project: Vec<String>,
    },

    /// Install amisgitpm with amisgitpm, check that everything is in place
    Bootstrap,
}
