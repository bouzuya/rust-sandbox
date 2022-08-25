#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSelector {
    pub collection_id: String,
    pub all_descendants: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: CollectionSelector =
            serde_json::from_str(r#"{"collectionId":"messages as m1","allDescendants":false}"#)?;
        assert_eq!(
            deserialized,
            CollectionSelector {
                collection_id: "messages as m1".to_owned(),
                all_descendants: false
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&CollectionSelector {
                collection_id: "messages as m1".to_owned(),
                all_descendants: false
            })?,
            r#"{"collectionId":"messages as m1","allDescendants":false}"#
        );
        Ok(())
    }
}
