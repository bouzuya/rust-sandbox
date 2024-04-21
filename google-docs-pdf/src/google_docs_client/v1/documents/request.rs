use crate::google_docs_client::v1::documents::Range;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(flatten)]
    pub request: Option<RequestRequest>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestRequest {
    // TODO: ...
    InsertText(InsertTextRequest),
    // TODO:
    DeleteContentRange(DeleteContentRangeRequest),
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteContentRangeRequest {
    pub range: Option<Range>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextRequest {
    pub text: Option<String>,
    #[serde(flatten)]
    pub insertion_location: Option<InsertTextRequestInsertionLocation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InsertTextRequestInsertionLocation {
    Location(Location),
    EndOfSegmentLocation(EndOfSegmentLocation),
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub segment_id: Option<String>,
    pub index: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfSegmentLocation {
    pub segment_id: Option<String>,
}
