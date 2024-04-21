use crate::google_docs_client::v1::documents::Dimension;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#size>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
}
