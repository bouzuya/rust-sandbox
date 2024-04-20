use super::text_style_suggestion_state::TextStyleSuggestionState;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#nestinglevelsuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NestingLevelSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_alignment_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glyph_type_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glyph_format_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glyph_symbol_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_first_line_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_start_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style_suggestion_state: Option<TextStyleSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_number_suggested: Option<bool>,
}
