use std::collections::HashMap;

use crate::google_docs_client::v1::documents::{
    StructuralElement, SuggestedTableCellStyle, TableCellStyle,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablecell>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<StructuralElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_style: Option<TableCellStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_table_cell_style_changes: Option<HashMap<String, SuggestedTableCellStyle>>,
}
