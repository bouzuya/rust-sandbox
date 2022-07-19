use limited_date_time::Instant;

use crate::{IssueCommentId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentDeleted {
    pub(super) at: Instant,
    pub(super) issue_comment_id: IssueCommentId,
    pub(super) version: Version,
}

// TODO: impl Display for IssueCommentCreated
// TODO: impl From<IssueCommentCreated> for String
// TODO: impl FromStr for IssueCommentCreated
// TODO: impl TryFrom<String> for IssueCommentCreated

#[cfg(test)]
mod tests {
    use super::*;

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
