pub mod attribute;
pub mod entity;
pub mod event;

use std::collections::BTreeSet;

use limited_date_time::Instant;

use crate::{IssueCommentId, IssueId, Version};

pub use self::event::Event;
use self::{
    attribute::IssueCommentText,
    entity::IssueComment,
    event::{IssueCommentCreated, IssueCommentDeleted, IssueCommentUpdated},
};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("entity {0}")]
    Entity(#[from] self::entity::Error),
    #[error("InvalidEventSequence")]
    InvalidEventSequence,
    #[error("NextVersion")]
    NoNextVersion,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentAggregate {
    events: Vec<Event>,
    issue_comment: IssueComment,
    version: Version,
}

impl IssueCommentAggregate {
    pub fn from_events(events: &[Event]) -> Result<Self> {
        let _ = Self::check_aggregate_id(events)?;
        let first_event = Self::check_first_event(events)?;
        let version = Self::check_version(events)?;
        let mut issue_comment = IssueComment::from_event(first_event.clone());
        for event in events.iter().skip(1) {
            issue_comment = match event {
                Event::Created(_) => Err(Error::InvalidEventSequence),
                Event::Deleted(_) => Ok(issue_comment.delete()?),
                Event::Updated(IssueCommentUpdated { text, .. }) => {
                    Ok(issue_comment.update(text.clone())?)
                }
            }?;
        }
        Ok(Self {
            events: vec![],
            issue_comment,
            version,
        })
    }

    pub fn new(
        at: Instant,
        issue_comment_id: IssueCommentId,
        issue_id: IssueId,
        text: IssueCommentText,
    ) -> Result<Self> {
        let issue_comment =
            IssueComment::new(issue_comment_id.clone(), issue_id.clone(), text.clone());
        let version = Version::from(1_u64);
        let event = IssueCommentCreated {
            at,
            issue_comment_id,
            issue_id,
            text,
            version,
        };
        let events = vec![event.into()];
        let aggregate = Self {
            events,
            issue_comment,
            version,
        };
        Ok(aggregate)
    }

    pub fn delete(&self, at: Instant) -> Result<Self> {
        let issue_comment = self.issue_comment.delete()?;
        let version = self.version.next().ok_or(Error::NoNextVersion)?;
        let event = IssueCommentDeleted {
            at,
            version,
            issue_comment_id: self.issue_comment.id().clone(),
        }
        .into();
        let events = [self.events.as_slice(), &[event]].concat();
        Ok(Self {
            events,
            issue_comment,
            version,
        })
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn id(&self) -> &IssueCommentId {
        self.issue_comment.id()
    }

    pub fn truncate_events(&self) -> Self {
        Self {
            events: vec![],
            issue_comment: self.issue_comment.clone(),
            version: self.version,
        }
    }

    pub fn update(&self, text: IssueCommentText, at: Instant) -> Result<Self> {
        let issue_comment = self.issue_comment.update(text.clone())?;
        let version = self.version.next().ok_or(Error::NoNextVersion)?;
        let event = IssueCommentUpdated {
            at,
            issue_comment_id: self.issue_comment.id().clone(),
            text,
            version,
        }
        .into();
        let events = [self.events.as_slice(), &[event]].concat();
        Ok(Self {
            events,
            issue_comment,
            version,
        })
    }

    pub fn version(&self) -> Version {
        self.version
    }

    fn check_aggregate_id(events: &[Event]) -> Result<&IssueCommentId> {
        let set = events
            .iter()
            .map(|e| e.issue_comment_id())
            .collect::<BTreeSet<&IssueCommentId>>();
        if set.len() == 1 {
            Ok(set.iter().next().expect("the set contains an element"))
        } else {
            Err(Error::InvalidEventSequence)
        }
    }

    fn check_first_event(events: &[Event]) -> Result<&event::IssueCommentCreated> {
        match events.first() {
            Some(event) => match event {
                Event::Created(event) => Ok(event),
                Event::Deleted(_) => Err(Error::InvalidEventSequence),
                Event::Updated(_) => Err(Error::InvalidEventSequence),
            },
            None => Err(Error::InvalidEventSequence),
        }
    }

    fn check_version(events: &[Event]) -> Result<Version> {
        let versions = events.iter().map(|e| e.version()).collect::<Vec<Version>>();
        let mut prev = *versions.first().ok_or(Error::InvalidEventSequence)?;
        for curr in versions.into_iter().skip(1) {
            if curr != prev.next().ok_or(Error::InvalidEventSequence)? {
                return Err(Error::InvalidEventSequence);
            }
            prev = curr;
        }
        Ok(prev)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    // TODO: from_events test

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let aggregate = IssueCommentAggregate::new(
            at,
            issue_comment_id.clone(),
            issue_id.clone(),
            text.clone(),
        )?;
        assert_eq!(
            aggregate.events(),
            &vec![IssueCommentCreated {
                at,
                issue_comment_id,
                issue_id,
                text,
                version: Version::from(1_u64),
            }
            .into()]
        );
        Ok(())
    }

    #[test]
    fn delete_test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let aggregate = IssueCommentAggregate::new(at, issue_comment_id.clone(), issue_id, text)?
            .truncate_events();

        let at = Instant::now();
        let deleted = aggregate.delete(at)?;
        assert_eq!(
            deleted.events(),
            &vec![IssueCommentDeleted {
                at,
                issue_comment_id,
                version: Version::from(2_u64),
            }
            .into()]
        );
        Ok(())
    }

    #[test]
    fn id_test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let aggregate = IssueCommentAggregate::new(at, issue_comment_id.clone(), issue_id, text)?;
        assert_eq!(aggregate.id(), &issue_comment_id);
        Ok(())
    }

    #[test]
    fn update_test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let aggregate = IssueCommentAggregate::new(at, issue_comment_id.clone(), issue_id, text)?
            .truncate_events();

        let at = Instant::now();
        let text = IssueCommentText::from_str("text2")?;
        let updated = aggregate.update(text.clone(), at)?;
        assert_eq!(
            updated.events(),
            &vec![IssueCommentUpdated {
                at,
                issue_comment_id,
                version: Version::from(2_u64),
                text
            }
            .into()]
        );
        Ok(())
    }

    #[test]
    fn version_test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_comment_id = IssueCommentId::generate();
        let issue_id = IssueId::from_str("123")?;
        let text = IssueCommentText::from_str("text")?;
        let aggregate = IssueCommentAggregate::new(at, issue_comment_id, issue_id, text)?;
        assert_eq!(aggregate.version(), Version::from(1_u64));
        Ok(())
    }
}
