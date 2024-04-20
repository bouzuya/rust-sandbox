use super::{
    paragraph_style::ParagraphStyle,
    paragraph_style_suggestion_state::ParagraphStyleSuggestionState,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestedparagraphstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedParagraphStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_style: Option<ParagraphStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_style_suggestion_state: Option<ParagraphStyleSuggestionState>,
}
