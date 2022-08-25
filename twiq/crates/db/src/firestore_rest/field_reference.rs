#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldReference {
    pub field_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: FieldReference =
            serde_json::from_str(r#"{"fieldPath":"max(messages.time) as max_time"}"#)?;
        assert_eq!(
            deserialized,
            FieldReference {
                field_path: "max(messages.time) as max_time".to_owned()
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&FieldReference {
                field_path: "max(messages.time) as max_time".to_owned()
            })?,
            r#"{"fieldPath":"max(messages.time) as max_time"}"#
        );
        Ok(())
    }
}
