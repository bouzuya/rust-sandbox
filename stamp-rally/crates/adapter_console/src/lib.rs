use use_case::CreateStampRallyUseCase;

pub fn run<C: CreateStampRallyUseCase>(create_stamp_rally_use_case: C) -> anyhow::Result<()> {
    let stamp_rally_id = create_stamp_rally_use_case.handle()?;
    println!("Hello, StampRally! (ID: {})", stamp_rally_id);
    Ok(())
}
