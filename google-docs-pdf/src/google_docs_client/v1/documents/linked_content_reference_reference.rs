use crate::google_docs_client::v1::documents::SheetsChartReference;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#linkedcontentreference>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkedContentReferenceReference {
    SheetsChartReference(SheetsChartReference),
}
