use crate::google_docs_client::v1::documents::StructuralElement;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tableofcontents>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableOfContents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<StructuralElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
}
