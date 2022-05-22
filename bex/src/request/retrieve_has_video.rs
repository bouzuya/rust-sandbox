use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum RetrieveHasVideo {
    #[serde(rename = "0")]
    Default,
    #[serde(rename = "1")]
    HasVideo,
    #[serde(rename = "2")]
    IsVideo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieve_item_status() -> anyhow::Result<()> {
        let status: RetrieveHasVideo = serde_json::from_str(r#""0""#)?;
        assert_eq!(status, RetrieveHasVideo::Default);
        let status: RetrieveHasVideo = serde_json::from_str(r#""1""#)?;
        assert_eq!(status, RetrieveHasVideo::HasVideo);
        let status: RetrieveHasVideo = serde_json::from_str(r#""2""#)?;
        assert_eq!(status, RetrieveHasVideo::IsVideo);
        Ok(())
    }
}
