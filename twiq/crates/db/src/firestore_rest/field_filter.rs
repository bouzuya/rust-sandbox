use super::{FieldOperator, FieldReference, Value};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FieldFilter {
    pub field: FieldReference,
    pub op: FieldOperator,
    pub value: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: FieldFilter = serde_json::from_str(
            r#"{"field":{"fieldPath":"a"},"op":"EQUAL","value":{"stringValue":"b"}}"#,
        )?;
        assert_eq!(
            deserialized,
            FieldFilter {
                field: FieldReference {
                    field_path: "a".to_owned()
                },
                op: FieldOperator::Equal,
                value: Value::String("b".to_owned())
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&FieldFilter {
                field: FieldReference {
                    field_path: "a".to_owned()
                },
                op: FieldOperator::Equal,
                value: Value::String("b".to_owned())
            })?,
            r#"{"field":{"fieldPath":"a"},"op":"EQUAL","value":{"stringValue":"b"}}"#
        );
        Ok(())
    }
}
