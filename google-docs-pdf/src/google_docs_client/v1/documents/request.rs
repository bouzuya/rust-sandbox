use crate::google_docs_client::v1::documents::Range;
use crate::google_docs_client::v1::documents::Size;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#request>
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
    // TODO: ...
    DeleteContentRange(DeleteContentRangeRequest),
    InsertInlineImage(InsertInlineImageRequest),
    // TODO: ...
}

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#insertinlineimagerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertInlineImageRequest {
    pub uri: Option<String>,
    pub object_size: Option<Size>,
    #[serde(flatten)]
    pub insertion_location: Option<InsertInlineImageRequestInsertionLocation>,
}

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#insertinlineimagerequest>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InsertInlineImageRequestInsertionLocation {
    Location(Location),
    EndOfSegmentLocation(EndOfSegmentLocation),
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
