use crate::data::{Token, TwitterClientKey};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Credential {
    pub client: TwitterClientKey,
    pub token: Token,
}
