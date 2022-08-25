use super::FieldReference;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Projection {
    pub fields: Vec<FieldReference>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: Projection =
            serde_json::from_str(r#"{"fields":[{"fieldPath":"max(messages.time) as max_time"}]}"#)?;
        assert_eq!(
            deserialized,
            Projection {
                fields: vec![FieldReference {
                    field_path: "max(messages.time) as max_time".to_owned()
                }]
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Projection {
                fields: vec![FieldReference {
                    field_path: "max(messages.time) as max_time".to_owned()
                }]
            })?,
            r#"{"fields":[{"fieldPath":"max(messages.time) as max_time"}]}"#
        );
        Ok(())
    }
}
