use crate::google_docs_client::v1::documents::Range;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#namedrange>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_range_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<Range>>,
}
