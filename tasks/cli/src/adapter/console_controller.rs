use std::rc::Rc;

use entity::TaskId;
use structopt::StructOpt;
use use_case::{
    AddUseCase, CompleteUseCase, ListPresenter, ListUseCase, RemoveUseCase, TaskRepository,
};

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
    list_presenter: Rc<dyn ListPresenter>,
    repository: Rc<dyn TaskRepository>,
}

impl ConsoleController {
    pub fn new(list_presenter: Rc<dyn ListPresenter>, repository: Rc<dyn TaskRepository>) -> Self {
        Self {
            list_presenter,
            repository,
        }
    }

    pub fn run(&self) {
        let opt = Opt::from_args();
        match opt.subcommand {
            Subcommand::Add { text } => AddUseCase::new(self.repository.clone()).handle(text),
            Subcommand::Complete { id } => {
                let id = TaskId::from(id);
                let use_case = CompleteUseCase::new(self.repository.clone());
                use_case.handle(id);
            }
            Subcommand::List { all } => {
                ListUseCase::new(self.list_presenter.clone(), self.repository.clone()).handle(all)
            }
            Subcommand::Remove { id } => {
                let id = TaskId::from(id);
                let use_case = RemoveUseCase::new(self.repository.clone());
                use_case.handle(id);
            }
        }
    }
}
