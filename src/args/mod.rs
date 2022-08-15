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
    /// Install a new git repo
    Install {
        #[clap(value_parser)]
        /// the git repo url
        url: String,
    },
    /// Update package(s)
    Update {
        #[clap(value_parser)]
        /// An optional package name to update independently.
        /// If not provided all packages are updated
        package: Option<String>,
    },
    /// Uninstall a package
    Uninstall {
        #[clap(value_parser)]
        /// The package name to uninstall
        package: String,
    },
    /// Remove all srcs with no project associated
    Cleanup {},
    /// Get info about a package
    Info {
        #[clap(value_parser)]
        /// The package name to get info about
        package: String,
    },
    /// Show the list of installed applications and their version
    List {},
    /// Install amisgitpm with amisgitpm, check that everything is in place
    Bootstrap {},
}
