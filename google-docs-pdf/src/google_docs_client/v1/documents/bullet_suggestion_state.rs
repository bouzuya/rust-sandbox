use crate::google_docs_client::v1::documents::TextStyleSuggestionState;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#bulletsuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
}
