mod adapter;

use self::adapter::{ConsoleGateway, JsonTaskDataSource};
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    let repository = Rc::new(JsonTaskDataSource::new()?);
    let controller = ConsoleGateway::new(repository);
    controller.run()
}
