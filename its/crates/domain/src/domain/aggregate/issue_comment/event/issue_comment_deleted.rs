use limited_date_time::Instant;

use crate::{IssueCommentId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentDeleted {
    pub(super) at: Instant,
    pub(super) issue_comment_id: IssueCommentId,
    pub(super) version: Version,
}

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

// TODO: impl TryFrom<IssueCommentDeletedJson> for IssueCommentDeleted

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: From<IssueCommentDeleted> for IssueCommentDeletedJson

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
