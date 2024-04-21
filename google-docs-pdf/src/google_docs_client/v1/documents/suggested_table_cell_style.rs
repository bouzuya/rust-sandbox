use crate::google_docs_client::v1::documents::{TableCellStyle, TableCellStyleSuggestionState};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestedtablecellstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedTableCellStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_style: Option<TableCellStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_style_suggestion_state: Option<TableCellStyleSuggestionState>,
}
