mod adapter;

use adapter::{ConsoleController, ConsolePresenter, JsonTaskDataSource};
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    let presenter = ConsolePresenter::new();
    let repository = Rc::new(JsonTaskDataSource::new()?);
    let controller = ConsoleController::new(presenter, repository);
    controller.run()
}
