use super::{
    embedded_object_suggestion_state::EmbeddedObjectSuggestionState,
    positioned_object_positioning_suggestion_state::PositionedObjectPositioningSuggestionState,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#positionedobjectpropertiessuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionedObjectPropertiesSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positioning_suggestion_state: Option<PositionedObjectPositioningSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_object_suggestion_state: Option<EmbeddedObjectSuggestionState>,
}
