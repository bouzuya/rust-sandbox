pub mod attribute;
pub mod entity;
pub mod event;

use crate::Version;

pub use self::event::Event;
use self::{entity::IssueComment, event::IssueCommentUpdated};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
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
        let first_event = match events.first() {
            Some(event) => match event {
                Event::Created(event) => Ok(event),
                Event::Deleted(_) => Err(Error::InvalidEventSequence),
                Event::Updated(_) => Err(Error::InvalidEventSequence),
            },
            None => Err(Error::InvalidEventSequence),
        }?;
        let mut issue_comment = IssueComment::from_event(first_event.clone());
        for event in events.iter().skip(1) {
            issue_comment = match event {
                Event::Created(_) => Err(Error::InvalidEventSequence),
                Event::Deleted(_) => todo!(),
                Event::Updated(IssueCommentUpdated { text, .. }) => {
                    Ok(issue_comment.update(text.clone()))
                }
            }?;
        }
        let last_event = events.last().expect("empty event sequence").clone();
        Ok(Self {
            events: vec![],
            issue_comment,
            version: last_event.version(),
        })
    }
}
