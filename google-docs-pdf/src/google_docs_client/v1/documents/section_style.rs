use crate::google_docs_client::v1::documents::{
    ColumnSeparatorStyle, ContentDirection, Dimension, SectionColumnProperties, SectionType,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#sectionstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_properties: Option<Vec<SectionColumnProperties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_separator_style: Option<ColumnSeparatorStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_direction: Option<ContentDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_header: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_footer: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_type: Option<SectionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_header_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_footer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page_header_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page_footer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_page_header_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_page_footer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_first_page_header_footer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number_start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flip_page_orientation: Option<bool>,
}
