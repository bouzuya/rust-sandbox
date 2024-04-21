/// <https://developers.google.com/docs/api/reference/rest/v1/documents#glyphtype>
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GlyphType {
    #[allow(clippy::enum_variant_names)]
    #[default]
    GlyphTypeUnspecified,
    None,
    Decimal,
    ZeroDecimal,
    UpperAlpha,
    Alpha,
    UpperRoman,
    Roman,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""GLYPH_TYPE_UNSPECIFIED""#,
                GlyphType::GlyphTypeUnspecified,
            ),
            (r#""NONE""#, GlyphType::None),
            (r#""DECIMAL""#, GlyphType::Decimal),
            (r#""ZERO_DECIMAL""#, GlyphType::ZeroDecimal),
            (r#""UPPER_ALPHA""#, GlyphType::UpperAlpha),
            (r#""ALPHA""#, GlyphType::Alpha),
            (r#""UPPER_ROMAN""#, GlyphType::UpperRoman),
            (r#""ROMAN""#, GlyphType::Roman),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
