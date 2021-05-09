use std::rc::Rc;
use structopt::StructOpt;
use tasks::{
    use_case::{AddUseCase, CompleteUseCase, ListUseCase, RemoveUseCase},
    TaskJsonRepository,
};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(about = "Adds a new task")]
    Add { text: String },
    #[structopt(about = "Completes the task")]
    Done { id: usize },
    #[structopt(about = "Lists tasks")]
    List {
        #[structopt(long = "all", help = "Prints all tasks")]
        all: bool,
    },
    #[structopt(about = "Removes the task")]
    Remove { id: usize },
}

fn main() {
    let opt = Opt::from_args();
    let repository = Rc::new(TaskJsonRepository::new());
    match opt.subcommand {
        Subcommand::Add { text } => AddUseCase::new(repository.clone()).add(text),
        Subcommand::Done { id } => CompleteUseCase::new(repository.clone()).complete(id),
        Subcommand::List { all } => ListUseCase::new(repository.clone()).list(all),
        Subcommand::Remove { id } => RemoveUseCase::new(repository.clone()).remove(id),
    }
}
