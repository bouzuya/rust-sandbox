use super::optional_color::OptionalColor;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#shading>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shading {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<OptionalColor>,
}
