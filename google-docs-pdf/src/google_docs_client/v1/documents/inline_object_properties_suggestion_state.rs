use crate::google_docs_client::v1::documents::EmbeddedObjectSuggestionState;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#inlineobjectpropertiessuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineObjectPropertiesSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_object_suggestion_state: Option<EmbeddedObjectSuggestionState>,
}
