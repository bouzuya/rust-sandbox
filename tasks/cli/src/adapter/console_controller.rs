use std::rc::Rc;

use entity::{TaskId, TaskText};
use structopt::StructOpt;
use use_case::{AddUseCase, CompleteUseCase, ListUseCase, RemoveUseCase, TaskRepository};

use super::ConsolePresenter;

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

pub struct ConsoleController {
    presenter: ConsolePresenter,
    repository: Rc<dyn TaskRepository>,
}

impl ConsoleController {
    pub fn new(presenter: ConsolePresenter, repository: Rc<dyn TaskRepository>) -> Self {
        Self {
            presenter,
            repository,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let opt = Opt::from_args();
        match opt.subcommand {
            Subcommand::Add { text } => {
                let text = TaskText::from(text);
                let use_case = AddUseCase::new(self.repository.clone());
                Ok(use_case.handle(text)?)
            }
            Subcommand::Complete { id } => {
                let id = TaskId::from(id);
                let use_case = CompleteUseCase::new(self.repository.clone());
                Ok(use_case.handle(id)?)
            }
            Subcommand::List { all } => {
                let use_case = ListUseCase::new(self.repository.clone());
                let output = use_case.handle(all)?;
                self.presenter.complete(&output);
                Ok(())
            }
            Subcommand::Remove { id } => {
                let id = TaskId::from(id);
                let use_case = RemoveUseCase::new(self.repository.clone());
                Ok(use_case.handle(id)?)
            }
        }
    }
}
