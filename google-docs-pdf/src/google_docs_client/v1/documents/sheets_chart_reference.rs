/// <https://developers.google.com/docs/api/reference/rest/v1/documents#sheetschartreference>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetsChartReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_id: Option<usize>,
}
