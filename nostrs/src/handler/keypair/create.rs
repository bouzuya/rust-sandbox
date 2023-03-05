use crate::keypair;

pub async fn create(private_key: String) -> anyhow::Result<()> {
    keypair::store(private_key)?;
    Ok(())
}
