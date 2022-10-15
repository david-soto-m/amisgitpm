use amisgitpm::{args::Cli, matcher, package_management::PackageManagerDefault};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let pm = PackageManagerDefault {  };
    matcher(args, pm);
}
