use nostr_sdk::prelude::{FromSkStr, Keys, ToBech32};

use crate::keypair;

pub async fn handle(private_key: bool) -> anyhow::Result<()> {
    let private_key_string = keypair::load()?;
    let keys = Keys::from_sk_str(private_key_string.as_str())?;
    if private_key {
        println!("{}", keys.secret_key()?.to_bech32()?);
    } else {
        println!("{}", keys.public_key().to_bech32()?);
    }
    Ok(())
}
