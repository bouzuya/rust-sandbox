use super::structural_element::StructuralElement;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#footer>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Footer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<StructuralElement>>,
}
