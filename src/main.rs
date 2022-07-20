use amisgitpm::args::{Cli, Commands};
use amisgitpm::gitutils;
use clap::Parser;

#[tokio::main]
async fn main() {
    let p = Cli::parse();
    println!("{p:?}");
    match p.com {
        Commands::Install { url, package_name } => gitutils::install(&url, package_name).await,
        _ => todo!(),
    };
}

/*Suggestions */
