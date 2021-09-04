use use_case::{CreateStampRallyUseCase, HasCreateStampRallyUseCase};

pub fn run<A>(application: A) -> anyhow::Result<()>
where
    A: HasCreateStampRallyUseCase,
{
    let stamp_rally_id = application.create_stamp_rally_use_case().handle()?;
    println!("Hello, StampRally! (ID: {})", stamp_rally_id);
    Ok(())
}
