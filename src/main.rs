use amisgitpm::{
    args::Cli,
    matcher,
    projects::ProjectTable,
    interaction::Interactor};
use clap::Parser;
use agpm_pm::ProjectManager;
use agpm_dirs::PMDirsImpl;
use agpm_abstract::*;

fn main() {
    let args = Cli::parse();
    let mut pm = ProjectManager::<PMDirsImpl, ProjectTable, Interactor>::new().unwrap();
    matcher(args, &mut pm);
}
