use crate::google_docs_client::v1::documents::ParagraphElementContent;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#paragraphelement>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<usize>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub content: Option<ParagraphElementContent>,
}
