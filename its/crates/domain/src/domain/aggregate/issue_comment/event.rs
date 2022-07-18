use limited_date_time::Instant;

use crate::{IssueCommentId, IssueId, Version};

use super::attribute::IssueCommentText;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentCreated {
    pub(super) at: Instant,
    pub(super) issue_comment_id: IssueCommentId,
    pub(super) issue_id: IssueId,
    pub(super) text: IssueCommentText,
    pub(super) version: Version,
}

// TODO: impl Display for IssueCommentCreated
// TODO: impl From<IssueCommentCreated> for String
// TODO: impl FromStr for IssueCommentCreated
// TODO: impl TryFrom<String> for IssueCommentCreated

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("comment")?;
        let version = Version::from(1_u64);
        let event = IssueCommentCreated {
            at,
            issue_comment_id: issue_comment_id.clone(),
            issue_id: issue_id.clone(),
            text: text.clone(),
            version,
        };
        assert_eq!(event.at, at);
        assert_eq!(event.issue_comment_id, issue_comment_id);
        assert_eq!(event.issue_id, issue_id);
        assert_eq!(event.text, text);
        assert_eq!(event.version, version);
        Ok(())
    }
}
