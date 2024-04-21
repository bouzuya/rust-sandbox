use crate::google_docs_client::v1::documents::Dimension;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablerowstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_row_height: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_overflow: Option<bool>,
}
