use super::crop_properties_suggestion_state::CropPropertiesSuggestionState;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#imagepropertiessuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagePropertiesSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_uri_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_uri_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_properties_suggestion_state: Option<CropPropertiesSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle_suggested: Option<bool>,
}
