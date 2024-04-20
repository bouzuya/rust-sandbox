/// <https://developers.google.com/docs/api/reference/rest/v1/documents#contentalignment>
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
pub enum ContentAlignment {
    #[allow(clippy::enum_variant_names)]
    #[default]
    ContentAlignmentUnspecified,
    #[allow(clippy::enum_variant_names)]
    ContentAlignmentUnsupported,
    Top,
    Middle,
    Bottom,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""CONTENT_ALIGNMENT_UNSPECIFIED""#,
                ContentAlignment::ContentAlignmentUnspecified,
            ),
            (
                r#""CONTENT_ALIGNMENT_UNSUPPORTED""#,
                ContentAlignment::ContentAlignmentUnsupported,
            ),
            (r#""TOP""#, ContentAlignment::Top),
            (r#""MIDDLE""#, ContentAlignment::Middle),
            (r#""BOTTOM""#, ContentAlignment::Bottom),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
