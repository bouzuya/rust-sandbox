use crate::google_docs_client::v1::documents::{
    PositionedObjectProperties, PositionedObjectPropertiesSuggestionState,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestedpositionedobjectproperties>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedPositionedObjectProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positioned_object_properties: Option<PositionedObjectProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positioned_object_properties_suggestion_state:
        Option<PositionedObjectPropertiesSuggestionState>,
}
