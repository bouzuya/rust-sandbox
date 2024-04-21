use crate::google_docs_client::v1::documents::{Dimension, TabStopAlignment};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tabstop>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabStop {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<TabStopAlignment>,
}
