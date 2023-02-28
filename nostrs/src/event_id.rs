use nostr_sdk::prelude::{EventId, FromBech32};

pub fn event_id_from_hex_or_bech32(s: &str) -> anyhow::Result<EventId> {
    Ok(EventId::from_hex(s).or_else(|_| EventId::from_bech32(s))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let hex = "027f70a8d7d0894dca117255d23c9a00b07ea99037f765b94f1ce71626b23375";
        let bech32 = "note1qflhp2xh6zy5mjs3wf2ay0y6qzc8a2vsxlmktw20rnn3vf4jxd6sr0xc5a";
        assert_eq!(
            event_id_from_hex_or_bech32(hex)?,
            event_id_from_hex_or_bech32(bech32)?
        );
        Ok(())
    }
}
