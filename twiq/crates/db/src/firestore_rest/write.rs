use super::{Document, DocumentMask, FieldTransform, Precondition};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Write {
    #[serde(rename_all = "camelCase")]
    Update {
        #[serde(skip_serializing_if = "Option::is_none")]
        current_document: Option<Precondition>,
        update: Document,
        #[serde(skip_serializing_if = "Option::is_none")]
        update_mask: Option<DocumentMask>,
        #[serde(skip_serializing_if = "Option::is_none")]
        update_transforms: Option<Vec<FieldTransform>>,
    },
    #[serde(rename_all = "camelCase")]
    Delete {
        #[serde(skip_serializing_if = "Option::is_none")]
        current_document: Option<Precondition>,
        delete: String,
    },
    // TODO:
    // Transform {
    //     current_document: Precondition,
    //     transform: DocumentTransform,
    // },
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::firestore_rest::{tests::serde_test, ServerValue, Value};

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            Write::Update {
                current_document: None,
                update: Document {
                    name: "projects/p/databases/(default)/documents/c/d".to_owned(),
                    fields: {
                        let mut map = HashMap::new();
                        map.insert("null".to_owned(), Value::Null);
                        map
                    },
                    create_time: Some("2022-08-19T22:53:42.480950Z".to_owned()),
                    update_time: Some("2022-08-19T22:53:42.480950Z".to_owned()),
                },
                update_mask: None,
                update_transforms: None,
            },
            r#"{"update":{"name":"projects/p/databases/(default)/documents/c/d","fields":{"null":{"nullValue":null}},"createTime":"2022-08-19T22:53:42.480950Z","updateTime":"2022-08-19T22:53:42.480950Z"}}"#,
        )?;
        serde_test(
            Write::Update {
                current_document: None,
                update: Document {
                    name: "projects/p/databases/(default)/documents/c/d".to_owned(),
                    fields: {
                        let mut map = HashMap::new();
                        map.insert("null".to_owned(), Value::Null);
                        map
                    },
                    create_time: Some("2022-08-19T22:53:42.480950Z".to_owned()),
                    update_time: Some("2022-08-19T22:53:42.480950Z".to_owned()),
                },
                update_mask: None,
                update_transforms: Some(vec![FieldTransform {
                    field_path: "f".to_owned(),
                    set_to_server_value: Some(ServerValue::RequestTime),
                }]),
            },
            r#"{"update":{"name":"projects/p/databases/(default)/documents/c/d","fields":{"null":{"nullValue":null}},"createTime":"2022-08-19T22:53:42.480950Z","updateTime":"2022-08-19T22:53:42.480950Z"},"updateTransforms":[{"fieldPath":"f","setToServerValue":"REQUEST_TIME"}]}"#,
        )?;
        serde_test(
            Write::Delete {
                current_document: None,
                delete: "123".to_owned(),
            },
            r#"{"delete":"123"}"#,
        )?;
        Ok(())
    }
}
