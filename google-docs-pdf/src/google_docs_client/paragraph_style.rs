use super::{
    alignment::Alignment, content_direction::ContentDirection, dimension::Dimension,
    named_style_type::NamedStyleType, paragraph_border::ParagraphBorder, shading::Shading,
    spacing_mode::SpacingMode, tab_stop::TabStop,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#paragraphstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_style_type: Option<NamedStyleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Alignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ContentDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing_mode: Option<SpacingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_above: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_below: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_between: Option<ParagraphBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top: Option<ParagraphBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom: Option<ParagraphBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left: Option<ParagraphBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right: Option<ParagraphBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_first_line: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_start: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_end: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_stops: Option<Vec<TabStop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_lines_together: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_with_next: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_widow_and_orphan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shading: Option<Shading>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_break_before: Option<bool>,
}
