use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    com: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install a new git repo
    Install {
        #[clap(value_parser)]
        /// the git repo url
        url: String,
        #[clap(value_parser)]
        /// The optional name for the package.
        /// If not set the repo name is used if available,
        ///if not a number will be appended to the repo name
        package_name: Option<String>,
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
}
