use crate::google_docs_client::v1::documents::CropProperties;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#imageproperties>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_properties: Option<CropProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
}
