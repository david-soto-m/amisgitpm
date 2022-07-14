use amisgitpm::args::Cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    let p = Cli::parse();
    println!("{p:?}");
    match p.com{
    }
}

/*Suggestions */
