use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum RetrieveHasImage {
    #[serde(rename = "0")]
    Default,
    #[serde(rename = "1")]
    HasImage,
    #[serde(rename = "2")]
    IsImage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieve_item_status() -> anyhow::Result<()> {
        let status: RetrieveHasImage = serde_json::from_str(r#""0""#)?;
        assert_eq!(status, RetrieveHasImage::Default);
        let status: RetrieveHasImage = serde_json::from_str(r#""1""#)?;
        assert_eq!(status, RetrieveHasImage::HasImage);
        let status: RetrieveHasImage = serde_json::from_str(r#""2""#)?;
        assert_eq!(status, RetrieveHasImage::IsImage);
        Ok(())
    }
}
