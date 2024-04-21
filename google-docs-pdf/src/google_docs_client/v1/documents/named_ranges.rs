use crate::google_docs_client::v1::documents::NamedRange;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#namedranges>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRanges {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_ranges: Option<Vec<NamedRange>>,
}
