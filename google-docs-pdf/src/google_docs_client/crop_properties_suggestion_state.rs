/// <https://developers.google.com/docs/api/reference/rest/v1/documents#croppropertiessuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CropPropertiesSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_left_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_right_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_top_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_bottom_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle_suggested: Option<bool>,
}
