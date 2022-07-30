use std::str::FromStr;

use limited_date_time::Instant;

use crate::{issue_comment_id, IssueCommentId, Version};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("instant error {0}")]
    Instant(#[from] limited_date_time::ParseInstantError),
    #[error("issue_comment_id error {0}")]
    IssueCommentId(#[from] issue_comment_id::Error),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentDeleted {
    pub(in crate::aggregate::issue_comment) at: Instant,
    pub(in crate::aggregate::issue_comment) issue_comment_id: IssueCommentId,
    pub(in crate::aggregate::issue_comment) version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IssueCommentDeletedJson {
    pub at: String,
    pub issue_comment_id: String,
    pub version: u64,
}

impl From<IssueCommentDeleted> for IssueCommentDeletedJson {
    fn from(event: IssueCommentDeleted) -> Self {
        Self {
            at: event.at.to_string(),
            issue_comment_id: event.issue_comment_id.to_string(),
            version: u64::from(event.version),
        }
    }
}

impl TryFrom<IssueCommentDeletedJson> for IssueCommentDeleted {
    type Error = Error;

    fn try_from(value: IssueCommentDeletedJson) -> Result<Self, Self::Error> {
        Ok(Self {
            at: Instant::from_str(value.at.as_str())?,
            issue_comment_id: IssueCommentId::from_str(value.issue_comment_id.as_str())?,
            version: Version::from(value.version),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_from_event_for_json() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let version = Version::from(1_u64);
        let event = IssueCommentDeleted {
            at,
            issue_comment_id: issue_comment_id.clone(),
            version,
        };
        assert_eq!(
            IssueCommentDeletedJson::from(event),
            IssueCommentDeletedJson {
                at: at.to_string(),
                issue_comment_id: issue_comment_id.to_string(),
                version: u64::from(version)
            }
        );
        Ok(())
    }

    #[test]
    fn impl_try_from_json_for_event() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let version = Version::from(1_u64);
        let json = IssueCommentDeletedJson {
            at: at.to_string(),
            issue_comment_id: issue_comment_id.to_string(),
            version: u64::from(version),
        };
        assert_eq!(
            IssueCommentDeleted::try_from(json)?,
            IssueCommentDeleted {
                at,
                issue_comment_id,
                version,
            }
        );
        Ok(())
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let version = Version::from(1_u64);
        let event = IssueCommentDeleted {
            at,
            issue_comment_id: issue_comment_id.clone(),
            version,
        };
        assert_eq!(event.at, at);
        assert_eq!(event.issue_comment_id, issue_comment_id);
        assert_eq!(event.version, version);
        Ok(())
    }
}
