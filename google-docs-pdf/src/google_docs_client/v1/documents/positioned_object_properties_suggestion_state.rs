use crate::google_docs_client::v1::documents::{
    EmbeddedObjectSuggestionState, PositionedObjectPositioningSuggestionState,
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
