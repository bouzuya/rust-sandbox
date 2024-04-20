use super::linked_content_reference_reference::LinkedContentReferenceReference;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#linkedcontentreference>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedContentReference {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub reference: Option<LinkedContentReferenceReference>,
}
