use crate::google_docs_client::v1::documents::{Dimension, PositionedObjectLayout};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#positionedobjectpositioning>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionedObjectPositioning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<PositionedObjectLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_offset: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_offset: Option<Dimension>,
}
