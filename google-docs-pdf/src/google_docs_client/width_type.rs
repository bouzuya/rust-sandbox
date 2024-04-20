/// <https://developers.google.com/docs/api/reference/rest/v1/documents#widthtype>
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
pub enum WidthType {
    #[allow(clippy::enum_variant_names)]
    #[default]
    WidthTypeUnspecified,
    EvenlyDistributed,
    FixedWidth,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""WIDTH_TYPE_UNSPECIFIED""#,
                WidthType::WidthTypeUnspecified,
            ),
            (r#""EVENLY_DISTRIBUTED""#, WidthType::EvenlyDistributed),
            (r#""FIXED_WIDTH""#, WidthType::FixedWidth),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
