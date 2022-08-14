use amisgitpm::{
    args::{Cli, Commands},
    build_suggestions::BuildSuggestions,
    gitutils::{GitUtilImpl, GitUtils},
    interaction::UserInstallInteractions,
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let p = Cli::parse();
    println!("{p:?}");
    match p.com {
        Commands::Install { url } => {
            GitUtilImpl::interactive_install::<BuildSuggestions, UserInstallInteractions>(&url)
                .unwrap()
        }
        _ => todo!(),
    };
}

/*Suggestions */
