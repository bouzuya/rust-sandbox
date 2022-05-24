use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Debug, Serialize)]
pub struct RetrieveRequest<'a> {
    pub consumer_key: &'a str,
    pub access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<RetrieveRequestState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<RetrieveRequestFavorite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<RetrieveRequestTag<'a>>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<RetrieveRequestContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<RetrieveRequestSort>,
    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<RetrieveRequestDetailType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub enum RetrieveRequestState {
    #[serde(rename = "unread")]
    Unread,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum RetrieveRequestFavorite {
    UnFavorited = 0,
    Favorited = 1,
}

#[derive(Debug)]
pub enum RetrieveRequestTag<'a> {
    Tagged(&'a str),
    Untagged,
}

impl<'a> Serialize for RetrieveRequestTag<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RetrieveRequestTag::Tagged(s) => serializer.serialize_str(s),
            RetrieveRequestTag::Untagged => serializer.serialize_str("_untagged_"),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum RetrieveRequestContentType {
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "image")]
    Image,
}

#[derive(Debug, Serialize)]
pub enum RetrieveRequestSort {
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "site")]
    Site,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum RetrieveRequestDetailType {
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "complete")]
    Complete,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieve_request_default() -> anyhow::Result<()> {
        let request = build_retrieve_request("consumer_key1", "access_token1");
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1"
}"#
        );
        Ok(())
    }

    #[test]
    fn retrieve_request_state() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");
        request.state = Some(RetrieveRequestState::Unread);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "state": "unread"
}"#
        );

        request.state = Some(RetrieveRequestState::Archive);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "state": "archive"
}"#
        );

        request.state = Some(RetrieveRequestState::All);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "state": "all"
}"#
        );
        Ok(())
    }

    #[test]
    fn retrieve_request_favorite() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.favorite = Some(RetrieveRequestFavorite::UnFavorited);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "favorite": 0
}"#
        );

        request.favorite = Some(RetrieveRequestFavorite::Favorited);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "favorite": 1
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_tag() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.tag = Some(RetrieveRequestTag::Tagged("tag1"));
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "tag": "tag1"
}"#
        );

        request.tag = Some(RetrieveRequestTag::Untagged);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "tag": "_untagged_"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_content_type() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.content_type = Some(RetrieveRequestContentType::Article);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "contentType": "article"
}"#
        );

        request.content_type = Some(RetrieveRequestContentType::Video);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "contentType": "video"
}"#
        );

        request.content_type = Some(RetrieveRequestContentType::Image);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "contentType": "image"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_sort() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.sort = Some(RetrieveRequestSort::Newest);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "sort": "newest"
}"#
        );

        request.sort = Some(RetrieveRequestSort::Oldest);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "sort": "oldest"
}"#
        );

        request.sort = Some(RetrieveRequestSort::Title);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "sort": "title"
}"#
        );

        request.sort = Some(RetrieveRequestSort::Site);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "sort": "site"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_detail_type() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.detail_type = Some(RetrieveRequestDetailType::Simple);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "detailType": "simple"
}"#
        );

        request.detail_type = Some(RetrieveRequestDetailType::Complete);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "detailType": "complete"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_search() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.search = Some("s");
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "search": "s"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_domain() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.domain = Some("example.com");
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "domain": "example.com"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_since() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.since = Some(1_234_567_890_u64);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "since": 1234567890
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_count() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.count = Some(10);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "count": 10
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_offset() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.offset = Some(10);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "offset": 10
}"#
        );

        Ok(())
    }

    fn build_retrieve_request<'a>(
        consumer_key: &'a str,
        access_token: &'a str,
    ) -> RetrieveRequest<'a> {
        RetrieveRequest {
            consumer_key,
            access_token,
            state: None,
            favorite: None,
            tag: None,
            content_type: None,
            sort: None,
            detail_type: None,
            search: None,
            domain: None,
            since: None,
            count: None,
            offset: None,
        }
    }
}
