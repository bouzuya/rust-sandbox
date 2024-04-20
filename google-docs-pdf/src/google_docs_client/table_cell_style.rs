use super::{
    content_alignment::ContentAlignment, dimension::Dimension, optional_color::OptionalColor,
    table_cell_border::TableCellBorder,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablecellstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCellStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<OptionalColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left: Option<TableCellBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right: Option<TableCellBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top: Option<TableCellBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom: Option<TableCellBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_left: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_right: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_top: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_bottom: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_alignment: Option<ContentAlignment>,
}
