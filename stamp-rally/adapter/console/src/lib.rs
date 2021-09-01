use use_case::create_stamp_rally;

pub fn run() -> anyhow::Result<()> {
    let stamp_rally_id = create_stamp_rally()?;
    println!("Hello, StampRally! (ID: {})", stamp_rally_id);
    Ok(())
}
