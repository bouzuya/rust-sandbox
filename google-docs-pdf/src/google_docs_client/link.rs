use super::link_destrination::LinkDestination;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#link>
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub destination: Option<LinkDestination>,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#"
{
  "url": "https://example.com"
}
"#,
                Link {
                    destination: Some(LinkDestination::Url(Some(
                        "https://example.com".to_string(),
                    ))),
                },
            ),
            (
                r#"
{
  "bookmarkId": "b.xxxxxxxxxxxx"
}
"#,
                Link {
                    destination: Some(LinkDestination::BookmarkId(Some(
                        "b.xxxxxxxxxxxx".to_string(),
                    ))),
                },
            ),
            (
                r#"
{
  "headingId": "h.xxxxxxxxxxxx"
}
"#,
                Link {
                    destination: Some(LinkDestination::HeadingId(Some(
                        "h.xxxxxxxxxxxx".to_string(),
                    ))),
                },
            ),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
