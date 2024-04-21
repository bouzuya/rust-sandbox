use crate::google_docs_client::v1::documents::{TableRow, TableStyle};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#table>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_rows: Option<Vec<TableRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_style: Option<TableStyle>,
}
