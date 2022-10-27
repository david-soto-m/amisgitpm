use agpm_abstract::*;
use agpm_dirs::PMDirsImpl;
use agpm_pm::ProjectManager;
use agpm_store::ProjectTable;
use amisgitpm::{args::Cli, matcher};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut pm = ProjectManager::<PMDirsImpl, ProjectTable<PMDirsImpl>, Interactor>::new().unwrap();
    matcher(args, &mut pm);
}
