use crate::google_docs_client::v1::documents::NamedStyle;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#namedstyles>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<NamedStyle>>,
}
