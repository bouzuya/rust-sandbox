/// <https://developers.google.com/docs/api/reference/rest/v1/documents#unit>
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
pub enum Unit {
    #[allow(clippy::enum_variant_names)]
    #[default]
    UnitUnspecified,
    Pt,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (r#""UNIT_UNSPECIFIED""#, Unit::UnitUnspecified),
            (r#""PT""#, Unit::Pt),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
