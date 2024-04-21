/// <https://developers.google.com/docs/api/reference/rest/v1/documents#section_type>
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
pub enum SectionType {
    #[allow(clippy::enum_variant_names)]
    #[default]
    SectionTypeUnspecified,
    Continuous,
    NextPage,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""SECTION_TYPE_UNSPECIFIED""#,
                SectionType::SectionTypeUnspecified,
            ),
            (r#""CONTINUOUS""#, SectionType::Continuous),
            (r#""NEXT_PAGE""#, SectionType::NextPage),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
