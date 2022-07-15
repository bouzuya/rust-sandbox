use limited_date_time::Instant;

use crate::{IssueCommentId, IssueCommentText, IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentCreated {
    at: Instant,
    issue_comment_id: IssueCommentId,
    issue_comment_text: IssueCommentText,
    issue_id: IssueId,
    version: Version,
}

impl IssueCommentCreated {
    pub(crate) fn from_trusted_data(
        at: Instant,
        issue_comment_id: IssueCommentId,
        issue_comment_text: IssueCommentText,
        issue_id: IssueId,
        version: Version,
    ) -> Self {
        Self::new(at, issue_comment_id, issue_comment_text, issue_id, version)
    }

    pub(crate) fn new(
        at: Instant,
        issue_comment_id: IssueCommentId,
        issue_comment_text: IssueCommentText,
        issue_id: IssueId,
        version: Version,
    ) -> Self {
        Self {
            at,
            issue_comment_id,
            issue_comment_text,
            issue_id,
            version,
        }
    }

    pub(crate) fn at(&self) -> Instant {
        self.at
    }

    pub(crate) fn issue_comment_id(&self) -> &IssueCommentId {
        &self.issue_comment_id
    }

    pub(crate) fn issue_id(&self) -> &IssueId {
        &self.issue_id
    }

    pub(crate) fn issue_comment_text(&self) -> &IssueCommentText {
        &self.issue_comment_text
    }

    pub(crate) fn version(&self) -> Version {
        self.version
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::IssueNumber;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let issue_comment_text = IssueCommentText::from_str("text")?;
        let issue_id = IssueId::new(IssueNumber::start_number());
        let version = Version::from(2_u64);
        let issue_updated = IssueCommentCreated::from_trusted_data(
            at,
            issue_comment_id.clone(),
            issue_comment_text.clone(),
            issue_id.clone(),
            version,
        );
        assert_eq!(issue_updated.at(), at);
        assert_eq!(issue_updated.issue_comment_id(), &issue_comment_id);
        assert_eq!(issue_updated.issue_comment_text(), &issue_comment_text);
        assert_eq!(issue_updated.issue_id(), &issue_id);
        assert_eq!(issue_updated.version(), version);
        Ok(())
    }
}
