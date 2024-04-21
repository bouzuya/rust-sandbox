/// <https://developers.google.com/docs/api/reference/rest/v1/documents#alignment>
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
pub enum Alignment {
    #[allow(clippy::enum_variant_names)]
    #[default]
    AlignmentUnspecified,
    Start,
    Center,
    End,
    Justified,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""ALIGNMENT_UNSPECIFIED""#,
                Alignment::AlignmentUnspecified,
            ),
            (r#""START""#, Alignment::Start),
            (r#""CENTER""#, Alignment::Center),
            (r#""END""#, Alignment::End),
            (r#""JUSTIFIED""#, Alignment::Justified),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
