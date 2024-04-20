/// <https://developers.google.com/docs/api/reference/rest/v1/documents#textstylesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyleSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_caps_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_font_family_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_offset_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_suggested: Option<bool>,
}
