use super::{FieldOperator, FieldReference, Value};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FieldFilter {
    pub field: FieldReference,
    pub op: FieldOperator,
    pub value: Value,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            FieldFilter {
                field: FieldReference {
                    field_path: "a".to_owned(),
                },
                op: FieldOperator::Equal,
                value: Value::String("b".to_owned()),
            },
            r#"{"field":{"fieldPath":"a"},"op":"EQUAL","value":{"stringValue":"b"}}"#,
        )?;
        Ok(())
    }
}
