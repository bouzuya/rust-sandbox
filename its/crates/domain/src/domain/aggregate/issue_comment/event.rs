pub mod issue_comment_created;
pub mod issue_comment_deleted;
pub mod issue_comment_updated;

use crate::{IssueCommentId, Version};

pub use self::{
    issue_comment_created::IssueCommentCreated, issue_comment_deleted::IssueCommentDeleted,
    issue_comment_updated::IssueCommentUpdated,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Created(IssueCommentCreated),
    Deleted(IssueCommentDeleted),
    Updated(IssueCommentUpdated),
}

macro_rules! impl_from_t1_for_t2 {
    ($t1:ty, $t2:ty, $variant:expr) => {
        impl From<$t1> for $t2 {
            fn from(event: $t1) -> Self {
                $variant(event)
            }
        }
    };
}

impl_from_t1_for_t2!(IssueCommentCreated, Event, Self::Created);
impl_from_t1_for_t2!(IssueCommentDeleted, Event, Self::Deleted);
impl_from_t1_for_t2!(IssueCommentUpdated, Event, Self::Updated);

// TODO: impl Display for Event
// TODO: impl From<Event> for String
// TODO: impl FromStr for Event
// TODO: impl TryFrom<String> for Event

impl Event {
    pub fn issue_comment_id(&self) -> &IssueCommentId {
        match self {
            Event::Created(IssueCommentCreated {
                issue_comment_id, ..
            }) => issue_comment_id,
            Event::Deleted(IssueCommentDeleted {
                issue_comment_id, ..
            }) => issue_comment_id,
            Event::Updated(IssueCommentUpdated {
                issue_comment_id, ..
            }) => issue_comment_id,
        }
    }

    pub fn version(&self) -> Version {
        match self {
            Event::Created(IssueCommentCreated { version, .. }) => *version,
            Event::Deleted(IssueCommentDeleted { version, .. }) => *version,
            Event::Updated(IssueCommentUpdated { version, .. }) => *version,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use limited_date_time::Instant;

    use crate::{
        aggregate::issue_comment::attribute::IssueCommentText, IssueCommentId, IssueId, Version,
    };

    use super::*;

    #[test]
    fn from_test() -> anyhow::Result<()> {
        let issue_comment_id = IssueCommentId::generate();
        let _ = Event::from(issue_comment_created(issue_comment_id.clone())?);
        let _ = Event::from(issue_comment_updated(issue_comment_id.clone())?);
        let _ = Event::from(issue_comment_deleted(issue_comment_id)?);
        Ok(())
    }

    #[test]
    fn issue_comment_id_test() -> anyhow::Result<()> {
        let issue_comment_id = IssueCommentId::generate();
        let events = vec![
            Event::from(issue_comment_created(issue_comment_id.clone())?),
            Event::from(issue_comment_updated(issue_comment_id.clone())?),
            Event::from(issue_comment_deleted(issue_comment_id.clone())?),
        ];
        for event in events {
            assert_eq!(event.issue_comment_id(), &issue_comment_id);
        }
        Ok(())
    }

    #[test]
    fn version_test() -> anyhow::Result<()> {
        let issue_comment_id = IssueCommentId::generate();
        let events = vec![
            Event::from(issue_comment_created(issue_comment_id.clone())?),
            Event::from(issue_comment_updated(issue_comment_id.clone())?),
            Event::from(issue_comment_deleted(issue_comment_id)?),
        ];
        for (i, event) in events.into_iter().enumerate() {
            assert_eq!(event.version(), Version::from(i as u64 + 1));
        }
        Ok(())
    }

    fn issue_comment_created(
        issue_comment_id: IssueCommentId,
    ) -> anyhow::Result<IssueCommentCreated> {
        let at = Instant::now();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let version = Version::from(1);
        Ok(IssueCommentCreated {
            at,
            issue_comment_id,
            issue_id,
            text,
            version,
        })
    }

    fn issue_comment_deleted(
        issue_comment_id: IssueCommentId,
    ) -> anyhow::Result<IssueCommentDeleted> {
        let at = Instant::now();
        let version = Version::from(3);
        Ok(IssueCommentDeleted {
            at,
            issue_comment_id,
            version,
        })
    }

    fn issue_comment_updated(
        issue_comment_id: IssueCommentId,
    ) -> anyhow::Result<IssueCommentUpdated> {
        let at = Instant::now();
        let text = IssueCommentText::from_str("text")?;
        let version = Version::from(2);
        Ok(IssueCommentUpdated {
            at,
            issue_comment_id,
            text,
            version,
        })
    }
}
