use super::{dimension::Dimension, width_type::WidthType};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablecolumnproperties>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableColumnProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_type: Option<WidthType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
}
