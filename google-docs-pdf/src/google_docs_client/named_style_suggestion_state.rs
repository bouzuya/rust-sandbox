use super::{
    named_style_type::NamedStyleType,
    paragraph_style_suggestion_state::ParagraphStyleSuggestionState,
    text_style_suggestion_state::TextStyleSuggestionState,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#namedstylesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedStyleSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_style_type: Option<NamedStyleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_style_suggestion_state: Option<ParagraphStyleSuggestionState>,
}
