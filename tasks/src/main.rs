mod driver;

use driver::{ListConsolePresenter, TaskJsonDataSource};
use std::rc::Rc;
use structopt::StructOpt;
use tasks::use_case::{AddUseCase, CompleteUseCase, ListUseCase, RemoveUseCase};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "add", about = "Adds a new task")]
    Add { text: String },
    #[structopt(name = "complete", about = "Completes the task")]
    Complete { id: usize },
    #[structopt(name = "list", about = "Lists tasks")]
    List {
        #[structopt(long = "all", help = "Prints all tasks")]
        all: bool,
    },
    #[structopt(name = "remove", about = "Removes the task")]
    Remove { id: usize },
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let list_presenter = Rc::new(ListConsolePresenter::new());
    let repository = Rc::new(TaskJsonDataSource::new()?);
    match opt.subcommand {
        Subcommand::Add { text } => AddUseCase::new(repository).handle(text),
        Subcommand::Complete { id } => CompleteUseCase::new(repository).handle(id),
        Subcommand::List { all } => ListUseCase::new(list_presenter, repository).handle(all),
        Subcommand::Remove { id } => RemoveUseCase::new(repository).handle(id),
    }
    Ok(())
}
