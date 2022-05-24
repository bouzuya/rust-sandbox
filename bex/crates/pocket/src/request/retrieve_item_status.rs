use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum RetrieveItemStatus {
    // 0: (default)
    #[serde(rename = "0")]
    Default,
    // 1: archived
    #[serde(rename = "1")]
    Archived,
    // 2: deleted
    #[serde(rename = "2")]
    Deleted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieve_item_status() -> anyhow::Result<()> {
        let status: RetrieveItemStatus = serde_json::from_str(r#""0""#)?;
        assert_eq!(status, RetrieveItemStatus::Default);
        let status: RetrieveItemStatus = serde_json::from_str(r#""1""#)?;
        assert_eq!(status, RetrieveItemStatus::Archived);
        let status: RetrieveItemStatus = serde_json::from_str(r#""2""#)?;
        assert_eq!(status, RetrieveItemStatus::Deleted);
        Ok(())
    }
}
