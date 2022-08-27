#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSelector {
    pub collection_id: String,
    pub all_descendants: bool,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            CollectionSelector {
                collection_id: "messages as m1".to_owned(),
                all_descendants: false,
            },
            r#"{"collectionId":"messages as m1","allDescendants":false}"#,
        )?;
        Ok(())
    }
}
