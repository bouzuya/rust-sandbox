use crate::google_docs_client::v1::documents::TextStyle;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#bullet>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bullet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
}
