use super::{
    named_style_type::NamedStyleType, paragraph_style::ParagraphStyle, text_style::TextStyle,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#namedstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_style_type: Option<NamedStyleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_style: Option<ParagraphStyle>,
}
