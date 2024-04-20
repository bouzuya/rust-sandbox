/// <https://developers.google.com/docs/api/reference/rest/v1/documents#contentdirection>
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
pub enum ContentDirection {
    #[allow(clippy::enum_variant_names)]
    #[default]
    ContentDirectionUnspecified,
    LeftToRight,
    RightToLeft,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""CONTENT_DIRECTION_UNSPECIFIED""#,
                ContentDirection::ContentDirectionUnspecified,
            ),
            (r#""LEFT_TO_RIGHT""#, ContentDirection::LeftToRight),
            (r#""RIGHT_TO_LEFT""#, ContentDirection::RightToLeft),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
