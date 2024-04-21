use crate::google_docs_client::v1::documents::{TableRowStyle, TableRowStyleSuggestionState};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestedtablerowstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedTableRowStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_row_style: Option<TableRowStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_row_style_suggestion_state: Option<TableRowStyleSuggestionState>,
}
