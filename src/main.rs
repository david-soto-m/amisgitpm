use amisgitpm::{
    args::{Cli, Commands},
    build_suggestions::BuildSuggestions,
    gitutils,
    interaction::UserInstallInteractions,
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let p = Cli::parse();
    println!("{p:?}");
    match p.com {
        Commands::Install { url } => {
            gitutils::interactive_install::<BuildSuggestions, UserInstallInteractions>(&url)
                .unwrap()
        }
        _ => todo!(),
    };
}

/*Suggestions */
