use argon2::password_hash::rand_core::{OsRng, RngCore};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SessionId([u8; 54]);

impl SessionId {
    pub fn generate() -> Self {
        let mut bytes = [0u8; 54];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        hex::encode(&self.0).fmt(f)
    }
}

impl std::str::FromStr for SessionId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        anyhow::ensure!(bytes.len() == 54);
        Ok(Self(bytes.as_slice().try_into()?))
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr as _};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "1231b72d839bb2691c76a61a3d466fa0b9afb2411f25fab0991f060543621738ef604b74babcb6243c566db7424cdceeddbb95ee23fc";
        let session_id = SessionId::from_str(s)?;
        assert_eq!(session_id.to_string(), s);

        assert_eq!(
            (0..100)
                .map(|_| SessionId::generate())
                .collect::<HashSet<_>>()
                .len(),
            100
        );

        Ok(())
    }
}
