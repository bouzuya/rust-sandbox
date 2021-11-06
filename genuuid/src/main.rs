use uuid::Uuid;

fn main() -> anyhow::Result<()> {
    let uuid = Uuid::new_v4();
    print!("{}", uuid);
    Ok(())
}
