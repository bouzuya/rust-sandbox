use super::color::Color;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#optionalcolor>
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalColor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}
