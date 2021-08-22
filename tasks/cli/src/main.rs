mod adapter;
mod driver;

use adapter::ConsoleController;
use driver::{ListConsolePresenter, TaskJsonDataSource};
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    let list_presenter = Rc::new(ListConsolePresenter::new());
    let repository = Rc::new(TaskJsonDataSource::new()?);
    let controller = ConsoleController::new(list_presenter, repository);
    controller.run();
    Ok(())
}
