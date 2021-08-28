mod adapter;

use adapter::{ConsoleController, ConsoleListPresenter, JsonTaskDataSource};
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    let list_presenter = Rc::new(ConsoleListPresenter::new());
    let repository = Rc::new(JsonTaskDataSource::new()?);
    let controller = ConsoleController::new(list_presenter, repository);
    controller.run()
}
