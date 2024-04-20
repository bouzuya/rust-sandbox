/// <https://developers.google.com/docs/api/reference/rest/v1/documents#positionedobjectpositioningsuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionedObjectPositioningSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_offset_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_offset_suggested: Option<bool>,
}
