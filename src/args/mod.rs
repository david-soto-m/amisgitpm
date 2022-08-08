use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub com: Commands,
}

#[derive(Subcommand, Debug, Clone)]
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
    /// get info about a package
    Info {
        #[clap(value_parser)]
        /// The package name to get info about
        package: String,
    },
    /// Show the list of installed applications and their version
    List {},
    /// Install amisgitpm with amisgitpm, check that everything is in place, try to do
    Bootstrap {},
}
