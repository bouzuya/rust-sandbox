pub mod attribute;
pub mod entity;
pub mod event;

use std::collections::BTreeSet;

use crate::{IssueCommentId, Version};

pub use self::event::Event;
use self::{entity::IssueComment, event::IssueCommentUpdated};

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
