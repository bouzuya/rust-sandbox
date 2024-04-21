use crate::google_docs_client::v1::documents::NestingLevelSuggestionState;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#listpropertiessuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPropertiesSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_levels_suggestion_states: Option<Vec<NestingLevelSuggestionState>>,
}
