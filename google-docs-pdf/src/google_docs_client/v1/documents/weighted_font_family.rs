/// <https://developers.google.com/docs/api/reference/rest/v1/documents#weightedfontfamily>
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedFontFamily {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<usize>,
}
