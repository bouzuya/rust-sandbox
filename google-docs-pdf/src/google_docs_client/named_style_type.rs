/// <https://developers.google.com/docs/api/reference/rest/v1/documents#namedstyletype>
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
pub enum NamedStyleType {
    #[allow(clippy::enum_variant_names)]
    #[default]
    NamedStyleTypeUnspecified,
    NormalText,
    Title,
    Subtitle,
    #[allow(non_camel_case_types)]
    Heading_1,
    #[allow(non_camel_case_types)]
    Heading_2,
    #[allow(non_camel_case_types)]
    Heading_3,
    #[allow(non_camel_case_types)]
    Heading_4,
    #[allow(non_camel_case_types)]
    Heading_5,
    #[allow(non_camel_case_types)]
    Heading_6,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""NAMED_STYLE_TYPE_UNSPECIFIED""#,
                NamedStyleType::NamedStyleTypeUnspecified,
            ),
            (r#""NORMAL_TEXT""#, NamedStyleType::NormalText),
            (r#""TITLE""#, NamedStyleType::Title),
            (r#""SUBTITLE""#, NamedStyleType::Subtitle),
            (r#""HEADING_1""#, NamedStyleType::Heading_1),
            (r#""HEADING_2""#, NamedStyleType::Heading_2),
            (r#""HEADING_3""#, NamedStyleType::Heading_3),
            (r#""HEADING_4""#, NamedStyleType::Heading_4),
            (r#""HEADING_5""#, NamedStyleType::Heading_5),
            (r#""HEADING_6""#, NamedStyleType::Heading_6),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
