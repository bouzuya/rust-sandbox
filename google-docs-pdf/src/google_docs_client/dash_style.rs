/// <https://developers.google.com/docs/api/reference/rest/v1/documents#dashstyle>
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
pub enum DashStyle {
    #[allow(clippy::enum_variant_names)]
    #[default]
    DashStyleUnspecified,
    Solid,
    Dot,
    Dash,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""DASH_STYLE_UNSPECIFIED"#,
                DashStyle::DashStyleUnspecified,
            ),
            (r#""SOLID"#, DashStyle::Solid),
            (r#""DOT"#, DashStyle::Dot),
            (r#""DASH"#, DashStyle::Dash),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
