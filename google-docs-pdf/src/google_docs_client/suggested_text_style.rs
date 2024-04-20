use super::{text_style::TextStyle, text_style_suggestion_state::TextStyleSuggestionState};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestedtextstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedTextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
}
