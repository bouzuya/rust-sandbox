#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub project_id: String,
    pub google_application_credentials: String,
}
