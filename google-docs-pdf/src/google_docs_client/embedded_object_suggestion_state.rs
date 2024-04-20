use super::{
    embedded_drawing_properties_suggestion_state::EmbeddedDrawingPropertiesSuggestionState,
    embedded_object_border_suggestion_state::EmbeddedObjectBorderSuggestionState,
    image_properties_suggestion_state::ImagePropertiesSuggestionState,
    linked_content_reference_suggestion_state::LinkedContentReferenceSuggestionState,
    size_suggestion_state::SizeSuggestionState,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#embeddedobjectsuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedObjectSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_drawing_properties_suggestion_state:
        Option<EmbeddedDrawingPropertiesSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_properties_suggestion_state: Option<ImagePropertiesSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_object_border_suggestion_state: Option<EmbeddedObjectBorderSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_suggestion_state: Option<SizeSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_content_reference_suggestion_state: Option<LinkedContentReferenceSuggestionState>,
}
