use amisgitpm::{args::Cli, matcher, package_management::PackageManager};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let pm = PackageManager {};
    matcher(args, pm);
}
