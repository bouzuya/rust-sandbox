use crate::google_docs_client::unit::Unit;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#dimension>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    magnitude: Option<serde_json::Number>,
    unit: Unit,
}

#[cfg(test)]
mod tests {
    use anyhow::Context as _;

    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#"
{
  "magnitude": 5.0,
  "unit": "PT"
}
            "#,
                Dimension {
                    magnitude: Some(
                        serde_json::Number::from_f64(5_f64)
                            .context("5_f64 is not valid json number")?,
                    ),
                    unit: Unit::Pt,
                },
            ),
            (
                r#"
{
  "magnitude": 5,
  "unit": "PT"
}
            "#,
                Dimension {
                    magnitude: Some(serde_json::Number::from(5)),
                    unit: Unit::Pt,
                },
            ),
            (
                r#"
{
  "unit": "PT"
}
            "#,
                Dimension {
                    magnitude: None,
                    unit: Unit::Pt,
                },
            ),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
