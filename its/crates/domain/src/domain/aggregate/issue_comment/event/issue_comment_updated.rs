use std::str::FromStr;

use limited_date_time::Instant;

use crate::{IssueCommentId, Version};

use super::super::attribute::IssueCommentText;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("instant error {0}")]
    Instant(#[from] limited_date_time::ParseInstantError),
    #[error("issue_comment_id error {0}")]
    IssueCommentId(#[from] crate::issue_comment_id::Error),
    #[error("text error {0}")]
    Text(#[from] crate::aggregate::issue_comment::attribute::issue_comment_text::Error),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentUpdated {
    pub(super) at: Instant,
    pub(super) issue_comment_id: IssueCommentId,
    pub(super) text: IssueCommentText,
    pub(super) version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IssueCommentUpdatedJson {
    pub at: String,
    pub issue_comment_id: String,
    pub text: String,
    pub version: u64,
}

impl From<IssueCommentUpdated> for IssueCommentUpdatedJson {
    fn from(event: IssueCommentUpdated) -> Self {
        Self {
            at: event.at.to_string(),
            issue_comment_id: event.issue_comment_id.to_string(),
            text: event.text.to_string(),
            version: u64::from(event.version),
        }
    }
}

impl TryFrom<IssueCommentUpdatedJson> for IssueCommentUpdated {
    type Error = Error;

    fn try_from(value: IssueCommentUpdatedJson) -> Result<Self, Self::Error> {
        Ok(Self {
            at: Instant::from_str(value.at.as_str())?,
            issue_comment_id: IssueCommentId::from_str(value.issue_comment_id.as_str())?,
            text: IssueCommentText::from_str(value.text.as_str())?,
            version: Version::from(value.version),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn impl_from_event_for_json() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let text = IssueCommentText::from_str("text")?;
        let version = Version::from(1_u64);
        let event = IssueCommentUpdated {
            at,
            issue_comment_id: issue_comment_id.clone(),
            text: text.clone(),
            version,
        };
        assert_eq!(
            IssueCommentUpdatedJson::from(event),
            IssueCommentUpdatedJson {
                at: at.to_string(),
                issue_comment_id: issue_comment_id.to_string(),
                text: text.to_string(),
                version: u64::from(version)
            }
        );
        Ok(())
    }

    #[test]
    fn impl_try_from_json_for_event() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let text = IssueCommentText::from_str("text")?;
        let version = Version::from(1_u64);
        let json = IssueCommentUpdatedJson {
            at: at.to_string(),
            issue_comment_id: issue_comment_id.to_string(),
            text: text.to_string(),
            version: u64::from(version),
        };
        assert_eq!(
            IssueCommentUpdated::try_from(json)?,
            IssueCommentUpdated {
                at,
                issue_comment_id,
                text,
                version,
            }
        );
        Ok(())
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let text = IssueCommentText::from_str("comment")?;
        let version = Version::from(1_u64);
        let event = IssueCommentUpdated {
            at,
            issue_comment_id: issue_comment_id.clone(),
            text: text.clone(),
            version,
        };
        assert_eq!(event.at, at);
        assert_eq!(event.issue_comment_id, issue_comment_id);
        assert_eq!(event.text, text);
        assert_eq!(event.version, version);
        Ok(())
    }
}
