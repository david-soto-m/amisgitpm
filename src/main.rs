use amisgitpm::{args::Cli, interaction::Interactor, matcher, package_management::PackageManager};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let pm = PackageManager {};
    let inter = Interactor {};
    matcher(args, pm, inter);
}
