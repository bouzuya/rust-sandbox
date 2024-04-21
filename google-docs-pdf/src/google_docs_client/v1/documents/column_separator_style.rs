/// <https://developers.google.com/docs/api/reference/rest/v1/documents#columnseparatorstyle>
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
pub enum ColumnSeparatorStyle {
    #[allow(clippy::enum_variant_names)]
    #[default]
    ColumnSeparatorStyleUnspecified,
    None,
    BetweenEachColumn,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""COLUMN_SEPARATOR_STYLE_UNSPECIFIED""#,
                ColumnSeparatorStyle::ColumnSeparatorStyleUnspecified,
            ),
            (r#""NONE""#, ColumnSeparatorStyle::None),
            (
                r#""BETWEEN_EACH_COLUMN""#,
                ColumnSeparatorStyle::BetweenEachColumn,
            ),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
