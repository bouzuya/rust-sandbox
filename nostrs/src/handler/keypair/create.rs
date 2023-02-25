use crate::keypair;

pub async fn handle(private_key: String) -> anyhow::Result<()> {
    keypair::store(private_key)?;
    Ok(())
}
