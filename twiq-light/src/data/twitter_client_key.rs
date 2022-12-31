#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TwitterClientKey {
    pub id: String,
    pub secret: String,
}
