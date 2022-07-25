/*use amisgitpm::args::{Cli, Commands};
use amisgitpm::gitutils;
use clap::Parser;
*/
use amisgitpm::dbmanager::{Permissions, Table};
use amisgitpm::dbs::BuildAux;
#[tokio::main]
async fn main() {
    /*
    let p = Cli::parse();
    println!("{p:?}");
    match p.com {
        Commands::Install { url } => gitutils::install(&url).await.unwrap(),
        _ => todo!(),
    };
    */
    let _table = Table::<BuildAux>::load("tests/db", Permissions::ReadOnly)
        .await
        .unwrap();
}

/*Suggestions */
