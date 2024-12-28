#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("authorization")]
    Authorization(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("deserialize")]
    Deserialize(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("error response {0}")]
    ErrorResponse(String),
    #[error("read response")]
    ReadResponse(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("request")]
    Request(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#Message>
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub name: Option<String>,
    // ...
    pub webpush: Option<WebpushConfig>,
    // ...
    pub token: Option<String>,
    // ...
}

/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#WebpushConfig>
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebpushConfig {
    // ...
    pub notification: Option<WebpushConfigNotification>,
    // ...
}

#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebpushConfigNotification {
    // ...
    pub body: Option<String>,
    pub data: Option<std::collections::HashMap<String, String>>,
    // ...
    pub icon: Option<String>,
    // ...
    pub require_interaction: Option<bool>,
    // ...
    pub title: Option<String>,
    // ...
}
