use crate::google_docs_client::v1::documents::{
    InlineObjectProperties, InlineObjectPropertiesSuggestionState,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestedinlineobjectproperties>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedInlineObjectProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_object_properties: Option<InlineObjectProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_object_properties_suggestion_state: Option<InlineObjectPropertiesSuggestionState>,
}
