use crate::google_docs_client::v1::documents::TableColumnProperties;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablestyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_column_properties: Option<Vec<TableColumnProperties>>,
}
