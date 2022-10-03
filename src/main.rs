use amisgitpm::{
    matcher,
    args::Cli,
    build_suggestions::BuildSuggestions,
    interaction::Interactor,
    package_management::PackageManager,
};

use clap::Parser;

fn main() {
    let args = Cli::parse();
    let pm = PackageManager {};
    let inter = Interactor{};
    matcher::<_, _, BuildSuggestions>(args, pm, inter);
}

/*Suggestions */
