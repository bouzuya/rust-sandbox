use crate::token::Token;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TwitterClientKey {
    pub id: String,
    pub secret: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Credential {
    pub client: TwitterClientKey,
    pub token: Token,
}
