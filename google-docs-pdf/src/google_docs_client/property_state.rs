/// <https://developers.google.com/docs/api/reference/rest/v1/documents#propertystate>
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
pub enum PropertyState {
    #[default]
    Rendered,
    NotRendered,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (r#""RENDERED""#, PropertyState::Rendered),
            (r#""NOT_RENDERED""#, PropertyState::NotRendered),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
