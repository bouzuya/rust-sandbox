/// <https://developers.google.com/docs/api/reference/rest/v1/documents#type>
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
pub enum Type {
    #[allow(clippy::enum_variant_names)]
    #[default]
    TypeUnspecified,
    PageNumber,
    PageCount,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (r#""TYPE_UNSPECIFIED""#, Type::TypeUnspecified),
            (r#""PAGE_NUMBER""#, Type::PageNumber),
            (r#""PAGE_COUNT""#, Type::PageCount),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
