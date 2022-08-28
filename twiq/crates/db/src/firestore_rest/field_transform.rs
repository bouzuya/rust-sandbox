use super::ServerValue;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldTransform {
    pub field_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_to_server_value: Option<ServerValue>,
    // TODO: increment
    // TODO: maximum
    // TODO: minimum
    // TODO: appendMissingElements
    // TODO: removeAllFromArray
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            FieldTransform {
                field_path: "at".to_owned(),
                set_to_server_value: Some(ServerValue::RequestTime),
            },
            r#"{"fieldPath":"at","setToServerValue":"REQUEST_TIME"}"#,
        )?;
        Ok(())
    }
}
