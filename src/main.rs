use amisgitpm::args::{Cli, Commands};
use clap::Parser;
use amisgitpm::gitutils;


#[tokio::main]
async fn main() {
    let p = Cli::parse();
    println!("{p:?}");
    match p.com {
        Commands::Install{url, package_name} => {
            gitutils::install(&url, package_name).await
        },
        _ => todo!(),
    };
}

/*Suggestions */
