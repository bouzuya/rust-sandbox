use super::glyph_type::GlyphType;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#nestinglevel>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NestingLevelGlyphKind {
    GlyphType(GlyphType),
    GlyphSymbol(String),
}
