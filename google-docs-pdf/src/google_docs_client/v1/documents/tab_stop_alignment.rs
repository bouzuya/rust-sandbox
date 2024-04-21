/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tabstopalignment>
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
pub enum TabStopAlignment {
    #[allow(clippy::enum_variant_names)]
    #[default]
    TabStopAlignmentUnspecified,
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
                r#""TAB_STOP_ALIGNMENT_UNSPECIFIED""#,
                TabStopAlignment::TabStopAlignmentUnspecified,
            ),
            (r#""START""#, TabStopAlignment::Start),
            (r#""CENTER""#, TabStopAlignment::Center),
            (r#""END""#, TabStopAlignment::End),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
