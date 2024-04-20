use super::{
    dimension::Dimension, embedded_object_border::EmbeddedObjectBorder,
    embedded_object_properties::EmbeddedObjectProperties,
    linked_content_reference::LinkedContentReference, size::Size,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#embeddedobject>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_object_border: Option<EmbeddedObjectBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_content_reference: Option<LinkedContentReference>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmbeddedObjectProperties>,
}
