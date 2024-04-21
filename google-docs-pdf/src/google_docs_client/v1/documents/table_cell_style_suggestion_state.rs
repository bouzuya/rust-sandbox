/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablecellstylesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCellStyleSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_left_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_right_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_top_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_bottom_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_alignment_suggested: Option<bool>,
}
