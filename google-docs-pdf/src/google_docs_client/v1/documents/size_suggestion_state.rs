/// <https://developers.google.com/docs/api/reference/rest/v1/documents#sizesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_suggested: Option<bool>,
}
