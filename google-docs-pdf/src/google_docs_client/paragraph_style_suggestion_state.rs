use super::shading_suggestion_state::ShadingSuggestionState;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#paragraphstylesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphStyleSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_style_type_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_spacing_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing_mode_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_above_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_below_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_between_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_first_line_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_start_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_end_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_lines_together_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_with_next_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_widow_and_orphan_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shading_suggestion_state: Option<ShadingSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_break_before_suggested: Option<bool>,
}
