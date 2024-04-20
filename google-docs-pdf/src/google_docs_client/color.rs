use super::rgb_color::RgbColor;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#color>
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgb_color: Option<RgbColor>,
}
