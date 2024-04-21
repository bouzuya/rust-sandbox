/// <https://developers.google.com/docs/api/reference/rest/v1/documents#cropproperties>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CropProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_right: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
}
