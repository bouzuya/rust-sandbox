/// <https://developers.google.com/docs/api/reference/rest/v1/documents#link>
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkDestination {
    Url(Option<String>),
    BookmarkId(Option<String>),
    HeadingId(Option<String>),
}
