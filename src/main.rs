use amisgitpm::args::Cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let p = Cli::parse();
    println!("{p:?}");
}

/*Suggestions */
