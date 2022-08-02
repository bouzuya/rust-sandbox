use limited_date_time::Instant;

use crate::{IssueCommentId, IssueId};

use super::attribute::IssueCommentText;
use super::event::IssueCommentCreated;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("already deleted {0} at {1}")]
    AlreadyDeleted(IssueCommentId, Instant),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

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

    pub fn delete(&self) -> Result<Self> {
        match self.deleted_at {
            Some(at) => Err(Error::AlreadyDeleted(self.id.clone(), at)),
            None => Ok(Self {
                id: self.id.clone(),
                issue_id: self.issue_id.clone(),
                text: self.text.clone(),
                deleted_at: Some(Instant::now()),
            }),
        }
    }

    pub fn id(&self) -> &IssueCommentId {
        &self.id
    }

    pub fn update(&self, text: IssueCommentText) -> Result<Self> {
        match self.deleted_at {
            Some(at) => Err(Error::AlreadyDeleted(self.id.clone(), at)),
            None => Ok(Self {
                id: self.id.clone(),
                issue_id: self.issue_id.clone(),
                text,
                deleted_at: self.deleted_at,
            }),
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
    fn from_event_test() -> anyhow::Result<()> {
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
        Ok(())
    }

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let issue_comment =
            IssueComment::new(issue_comment_id.clone(), issue_id.clone(), text.clone());
        assert_eq!(issue_comment.id, issue_comment_id);
        assert_eq!(issue_comment.issue_id, issue_id);
        assert_eq!(issue_comment.text, text);
        Ok(())
    }

    #[test]
    fn delete_test() -> anyhow::Result<()> {
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let issue_comment = IssueComment::new(issue_comment_id, issue_id, text);
        assert!(issue_comment.deleted_at.is_none());

        let deleted = issue_comment.delete()?;
        assert!(deleted.deleted_at.is_some());

        assert!(deleted.delete().is_err());

        Ok(())
    }

    #[test]
    fn id_test() -> anyhow::Result<()> {
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let issue_comment = IssueComment::new(issue_comment_id.clone(), issue_id, text);
        assert_eq!(issue_comment.id(), &issue_comment_id);
        Ok(())
    }

    #[test]
    fn update_test() -> anyhow::Result<()> {
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text1")?;
        let issue_comment = IssueComment::new(issue_comment_id, issue_id, text);
        assert_eq!(issue_comment.text.to_string(), "text1");

        let text = IssueCommentText::from_str("text2")?;
        let updated = issue_comment.update(text)?;
        assert_eq!(updated.text.to_string(), "text2");

        let deleted = updated.delete()?;
        let text = IssueCommentText::from_str("text3")?;
        assert!(deleted.update(text).is_err());

        Ok(())
    }
}
