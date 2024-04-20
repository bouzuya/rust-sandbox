/// <https://developers.google.com/docs/api/reference/rest/v1/documents#rgbcolor>
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RgbColor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue: Option<f64>,
}
