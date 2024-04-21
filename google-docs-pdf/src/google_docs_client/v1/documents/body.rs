use crate::google_docs_client::v1::documents::StructuralElement;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#body>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<StructuralElement>>,
}
