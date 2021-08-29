use console::ConsoleGateway;
use json::JsonTaskDataSource;
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    let repository = Rc::new(JsonTaskDataSource::new()?);
    let gateway = ConsoleGateway::new(repository);
    gateway.run()
}
