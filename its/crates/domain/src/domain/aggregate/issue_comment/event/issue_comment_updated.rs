use limited_date_time::Instant;

use crate::{IssueCommentId, Version};

use super::super::attribute::IssueCommentText;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentUpdated {
    pub(super) at: Instant,
    pub(super) issue_comment_id: IssueCommentId,
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
