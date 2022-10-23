use amisgitpm::{
    args::Cli,
    matcher,
    projects::ProjectTable,
    interaction::Interactor,
    dirutils::PMDirsImpl};
use clap::Parser;
use agpm_pm::ProjectManager;
use agpm_abstract::*;

fn main() {
    let args = Cli::parse();
    let mut pm = ProjectManager::<PMDirsImpl, ProjectTable, Interactor>::new().unwrap();
    matcher(args, &mut pm);
}
