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
    /// 1. git clone URL eg. `https://github.com/helix-editor/helix.git`
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

    /// Update package(s)
    Update {
        #[clap(value_parser)]
        /// An optional package name to update independently.
        /// If not provided all packages are updated
        package: Option<String>,
        #[clap(short, long)]
        // A flag to force update all packages configured to ask
        force: bool,
    },

    /// Get the last version of the package
    Restore {
        #[clap(value_parser)]
        /// An optional package name to update independently.
        /// If not provided all packages are updated
        package: String,
    },

    /// Uninstall a package
    Uninstall {
        #[clap(value_parser)]
        /// The package name to uninstall
        package: String,
    },

    /// Uninstall then install a package
    Reinstall {
        #[clap(value_parser)]
        /// The package name to reinstall
        package: String,
    },

    /// Run the build instructions of a package
    Rebuild {
        #[clap(value_parser)]
        /// The package name to rebuild
        package: String,
    },

    /// Remove all srcs with no project associated
    ///
    /// It is `O(N^2)`, with `N` the number of installed packages
    /// It is parallelized, and therefore is panicky rather than reporting
    Cleanup,

    /// Edit the configuration of a project
    Edit {
        #[clap(value_parser)]
        /// The name of package to edit
        package: String,
    },

    /// Show the list of installed applications and their version
    List {
        #[clap(value_parser)]
        /// Packages from which to get detailed information
        package: Vec<String>,
    },

    /// Install amisgitpm with amisgitpm, check that everything is in place
    Bootstrap,
}
