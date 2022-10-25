use agpm_abstract::*;
use agpm_dirs::PMDirsImpl;
use agpm_pm::ProjectManager;
use amisgitpm::{args::Cli, interaction::Interactor, matcher, projects::ProjectTable};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut pm = ProjectManager::<PMDirsImpl, ProjectTable, Interactor>::new().unwrap();
    matcher(args, &mut pm);
}
