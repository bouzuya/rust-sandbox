use super::{
    embedded_drawing_properties::EmbeddedDrawingProperties, image_properties::ImageProperties,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#embeddedobject>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EmbeddedObjectProperties {
    EmbeddedDrawingProperties(EmbeddedDrawingProperties),
    ImageProperties(ImageProperties),
}
