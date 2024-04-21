/// <https://developers.google.com/docs/api/reference/rest/v1/documents#spacingmode>
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
pub enum SpacingMode {
    #[allow(clippy::enum_variant_names)]
    #[default]
    SpacingModeUnspecified,
    NeverCollapse,
    CollapseLists,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""SPACING_MODE_UNSPECIFIED""#,
                SpacingMode::SpacingModeUnspecified,
            ),
            (r#""NEVER_COLLAPSE""#, SpacingMode::NeverCollapse),
            (r#""COLLAPSE_LISTS""#, SpacingMode::CollapseLists),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
