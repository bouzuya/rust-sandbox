use crate::google_docs_client::v1::documents::NamedStyleSuggestionState;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#namedstylessuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedStylesSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles_suggestion_states: Option<Vec<NamedStyleSuggestionState>>,
}
