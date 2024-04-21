use crate::google_docs_client::v1::documents::{
    BaselineOffset, Dimension, Link, OptionalColor, WeightedFontFamily,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#textstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_caps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<OptionalColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<OptionalColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_font_family: Option<WeightedFontFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_offset: Option<BaselineOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}
