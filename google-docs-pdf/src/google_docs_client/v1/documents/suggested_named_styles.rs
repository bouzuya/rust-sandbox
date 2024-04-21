use crate::google_docs_client::v1::documents::{NamedStyles, NamedStylesSuggestionState};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestednamedstyles>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedNamedStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_styles: Option<NamedStyles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_styles_suggestion_state: Option<NamedStylesSuggestionState>,
}
