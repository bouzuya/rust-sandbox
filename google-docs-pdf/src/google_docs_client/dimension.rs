use crate::google_docs_client::unit::Unit;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#dimension>
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    magnitude: Option<f64>,
    unit: Unit,
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
  "magnitude": 5,
  "unit": "PT"
}
            "#,
                Dimension {
                    magnitude: Some(5_f64),
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
