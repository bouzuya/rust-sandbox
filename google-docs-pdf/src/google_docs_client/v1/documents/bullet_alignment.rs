/// <https://developers.google.com/docs/api/reference/rest/v1/documents#bulletalignment>
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
pub enum BulletAlignment {
    #[allow(clippy::enum_variant_names)]
    #[default]
    BulletAlignmentUnspecified,
    Start,
    Center,
    End,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""BULLET_ALIGNMENT_UNSPECIFIED""#,
                BulletAlignment::BulletAlignmentUnspecified,
            ),
            (r#""START""#, BulletAlignment::Start),
            (r#""CENTER""#, BulletAlignment::Center),
            (r#""END""#, BulletAlignment::End),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
