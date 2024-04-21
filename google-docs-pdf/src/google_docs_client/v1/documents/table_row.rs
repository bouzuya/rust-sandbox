use std::collections::HashMap;

use crate::google_docs_client::v1::documents::{SuggestedTableRowStyle, TableCell, TableRowStyle};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablerow>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cells: Option<Vec<TableCell>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_row_style: Option<TableRowStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_table_row_style_changes: Option<HashMap<String, SuggestedTableRowStyle>>,
}
