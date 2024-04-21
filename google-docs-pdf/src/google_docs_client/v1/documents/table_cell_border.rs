use crate::google_docs_client::v1::documents::{DashStyle, Dimension, OptionalColor};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablecellborder>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCellBorder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<OptionalColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_style: Option<DashStyle>,
}