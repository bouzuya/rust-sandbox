/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestionsviewmode>
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
pub enum SuggestionsViewMode {
    #[default]
    DefaultForCurrentAccess,
    SuggestionsInline,
    PreviewSuggestionsAccepted,
    PreviewWithoutSuggestions,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""DEFAULT_FOR_CURRENT_ACCESS""#,
                SuggestionsViewMode::DefaultForCurrentAccess,
            ),
            (
                r#""SUGGESTIONS_INLINE""#,
                SuggestionsViewMode::SuggestionsInline,
            ),
            (
                r#""PREVIEW_SUGGESTIONS_ACCEPTED""#,
                SuggestionsViewMode::PreviewSuggestionsAccepted,
            ),
            (
                r#""PREVIEW_WITHOUT_SUGGESTIONS""#,
                SuggestionsViewMode::PreviewWithoutSuggestions,
            ),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
