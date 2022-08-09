use amisgitpm::args::{Cli, Commands};
use amisgitpm::gitutils;
use amisgitpm::interaction::UserInstallInteractions;
use clap::Parser;

#[tokio::main]
async fn main() {
    let p = Cli::parse();
    println!("{p:?}");
    match p.com {
        Commands::Install { url } => {
            gitutils::interactive_install::<UserInstallInteractions>(&url).unwrap()
        }
        _ => todo!(),
    };
}

/*Suggestions */
