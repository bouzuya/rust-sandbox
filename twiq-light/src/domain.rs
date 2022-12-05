#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MyTweet {
    pub id_str: String,
    pub at: String,
    pub text: String,
}
