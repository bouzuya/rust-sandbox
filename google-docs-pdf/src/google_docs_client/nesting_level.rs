use super::{
    bullet_alignment::BulletAlignment, dimension::Dimension,
    nesting_level_glyph_kind::NestingLevelGlyphKind, text_style::TextStyle,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#nestinglevel>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NestingLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_alignment: Option<BulletAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glyph_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_first_line: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_start: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_number: Option<usize>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub glyph_kind: Option<NestingLevelGlyphKind>,
}
