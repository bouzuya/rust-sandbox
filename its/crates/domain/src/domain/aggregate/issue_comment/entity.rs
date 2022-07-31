use limited_date_time::Instant;

use crate::{IssueCommentId, IssueId};

use super::attribute::IssueCommentText;
use super::event::IssueCommentCreated;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueComment {
    id: IssueCommentId,
    issue_id: IssueId,
    text: IssueCommentText,
    deleted_at: Option<Instant>,
}

impl IssueComment {
    pub fn from_event(event: IssueCommentCreated) -> Self {
        Self {
            id: event.issue_comment_id,
            issue_id: event.issue_id,
            text: event.text,
            deleted_at: None,
        }
    }

    pub fn new(id: IssueCommentId, issue_id: IssueId, text: IssueCommentText) -> Self {
        Self {
            id,
            issue_id,
            text,
            deleted_at: None,
        }
    }

    pub fn delete(&self) -> Self {
        match self.deleted_at {
            Some(_) => todo!(),
            None => Self {
                id: self.id.clone(),
                issue_id: self.issue_id.clone(),
                text: self.text.clone(),
                deleted_at: Some(Instant::now()),
            },
        }
    }

    pub fn update(&self, text: IssueCommentText) -> Self {
        if self.deleted_at.is_some() {
            todo!()
        }
        Self {
            id: self.id.clone(),
            issue_id: self.issue_id.clone(),
            text,
            deleted_at: self.deleted_at,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use limited_date_time::Instant;

    use crate::Version;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let version = Version::from(1_u64);
        let event = IssueCommentCreated {
            at,
            issue_comment_id: issue_comment_id.clone(),
            issue_id: issue_id.clone(),
            text: text.clone(),
            version,
        };
        let issue_comment = IssueComment::from_event(event);
        assert_eq!(issue_comment.id, issue_comment_id);
        assert_eq!(issue_comment.issue_id, issue_id);
        assert_eq!(issue_comment.text, text);
        // TODO
        Ok(())
    }

    // TODO: delete test
    // TODO: update test
}
