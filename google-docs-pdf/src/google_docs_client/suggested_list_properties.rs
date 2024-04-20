use super::{
    list_properties::ListProperties,
    list_properties_suggestion_state::ListPropertiesSuggestionState,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestedlistproperties>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedListProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_properties: Option<ListProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_properties_suggestion_state: Option<ListPropertiesSuggestionState>,
}
