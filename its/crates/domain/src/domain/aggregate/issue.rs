mod error;
mod event;

use limited_date_time::Instant;

pub use self::error::*;
pub use self::event::*;
use crate::IssueCreatedV2;
use crate::IssueDue;
use crate::IssueNumber;
use crate::IssueStatus;
use crate::IssueTitle;
use crate::IssueUpdated;
use crate::{
    domain::{entity::Issue, event::IssueFinished},
    IssueId, Version,
};

use super::IssueBlockLinkAggregate;
use super::IssueBlockLinkAggregateError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueAggregate {
    events: Vec<IssueAggregateEvent>,
    issue: Issue,
    version: Version,
}

impl IssueAggregate {
    pub fn from_events(events: &[IssueAggregateEvent]) -> Result<Self, IssueAggregateError> {
        let first_event = match events.first() {
            Some(event) => match event {
                IssueAggregateEvent::Created(event) => Ok(IssueCreatedV2::from_v1(event.clone())),
                IssueAggregateEvent::CreatedV2(event) => Ok(event.clone()),
                IssueAggregateEvent::Finished(_) => Err(IssueAggregateError::InvalidEventSequence),
                IssueAggregateEvent::Updated(_) => Err(IssueAggregateError::InvalidEventSequence),
            },
            None => Err(IssueAggregateError::InvalidEventSequence),
        }?;
        let version = first_event.version;
        let issue = Issue::from_event(first_event);
        let mut issue = IssueAggregate {
            events: vec![],
            issue,
            version,
        };
        for event in events.iter().skip(1) {
            match event {
                IssueAggregateEvent::Created(_) => {
                    return Err(IssueAggregateError::InvalidEventSequence);
                }
                IssueAggregateEvent::CreatedV2(_) => {
                    return Err(IssueAggregateError::InvalidEventSequence);
                }
                IssueAggregateEvent::Finished(IssueFinished {
                    at: _,
                    issue_id,
                    resolution,
                    version,
                }) => {
                    if issue.issue.id() != issue_id {
                        return Err(IssueAggregateError::InvalidEventSequence);
                    }
                    if issue.version.next() != Some(*version) {
                        return Err(IssueAggregateError::InvalidEventSequence);
                    }

                    // FIXME: Use resolution
                    issue = IssueAggregate {
                        events: vec![],
                        issue: issue
                            .issue
                            .finish()
                            .map_err(|_| IssueAggregateError::InvalidEventSequence)?,
                        version: *version,
                    }
                }
                IssueAggregateEvent::Updated(IssueUpdated {
                    at: _,
                    issue_id,
                    issue_due,
                    version,
                }) => {
                    if issue.issue.id() != issue_id {
                        return Err(IssueAggregateError::InvalidEventSequence);
                    }
                    if issue.version.next() != Some(*version) {
                        return Err(IssueAggregateError::InvalidEventSequence);
                    }

                    issue = IssueAggregate {
                        events: vec![],
                        issue: issue.issue.change_due(*issue_due),
                        version: *version,
                    }
                }
            }
        }
        Ok(issue)
    }

    pub fn new(
        at: Instant,
        issue_number: IssueNumber,
        issue_title: IssueTitle,
        issue_due: Option<IssueDue>,
    ) -> Result<Self, IssueAggregateError> {
        let issue_id = IssueId::new(issue_number);
        let issue = Issue::new(issue_id.clone(), issue_title.clone(), issue_due);
        let version = Version::from(1_u64);
        let event = IssueCreatedV2 {
            at,
            issue_id,
            issue_title,
            issue_due,
            version,
        }
        .into();
        let events = vec![event];
        let issue = IssueAggregate {
            events,
            issue,
            version,
        };
        Ok(issue)
    }

    pub fn finish(&self, at: Instant) -> Result<IssueAggregate, IssueAggregateError> {
        let updated_issue = self
            .issue
            .finish()
            .map_err(|_| IssueAggregateError::Unknown)?;
        let updated_version = self.version.next().ok_or(IssueAggregateError::Unknown)?;
        let event = IssueFinished {
            at,
            issue_id: self.id().clone(),
            resolution: None, // FIXME
            version: updated_version,
        }
        .into();
        let events = [self.events.as_slice(), &[event]].concat();
        Ok(IssueAggregate {
            events,
            issue: updated_issue,
            version: updated_version,
        })
    }

    pub fn update(
        &self,
        issue_due: Option<IssueDue>,
        at: Instant,
    ) -> Result<IssueAggregate, IssueAggregateError> {
        let updated_issue = self.issue.change_due(issue_due);
        let updated_version = self.version.next().ok_or(IssueAggregateError::Unknown)?;
        let event = IssueUpdated {
            at,
            issue_id: self.id().clone(),
            issue_due: updated_issue.due(),
            version: updated_version,
        }
        .into();
        let events = [self.events.as_slice(), &[event]].concat();
        Ok(IssueAggregate {
            events,
            issue: updated_issue,
            version: updated_version,
        })
    }

    pub fn truncate_events(self) -> Self {
        Self {
            events: vec![],
            issue: self.issue,
            version: self.version,
        }
    }

    pub fn events(&self) -> &Vec<IssueAggregateEvent> {
        &self.events
    }

    pub fn id(&self) -> &IssueId {
        self.issue.id()
    }

    pub fn status(&self) -> IssueStatus {
        self.issue.status()
    }

    pub fn title(&self) -> &IssueTitle {
        self.issue.title()
    }

    pub fn due(&self) -> Option<IssueDue> {
        self.issue.due()
    }

    pub fn block(
        &self,
        blocked_issue: IssueAggregate,
        at: Instant,
    ) -> Result<IssueBlockLinkAggregate, IssueBlockLinkAggregateError> {
        IssueBlockLinkAggregate::new(at, self.id().clone(), blocked_issue.id().clone())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let issue_number = IssueNumber::from_str("123")?;
        let issue_title = IssueTitle::from_str("title")?;
        let issue_due = Some(IssueDue::from_str("2021-02-03T04:05:06Z")?);
        let issue =
            IssueAggregate::new(Instant::now(), issue_number, issue_title.clone(), issue_due)?;
        assert_eq!(issue.id().issue_number(), issue_number);
        assert_eq!(issue.title(), &issue_title);
        assert_eq!(issue.due(), issue_due);
        Ok(())
    }

    #[test]
    fn finish_test() -> anyhow::Result<()> {
        let issue = IssueAggregate::new(
            Instant::now(),
            IssueNumber::from_str("123")?,
            IssueTitle::from_str("title")?,
            Some(IssueDue::from_str("2021-02-03T04:05:06Z")?),
        )?;
        let _ = issue.finish(Instant::now())?;
        // TODO: assert
        Ok(())
    }

    #[test]
    fn updaate_test() -> anyhow::Result<()> {
        let issue = IssueAggregate::new(
            Instant::now(),
            IssueNumber::from_str("123")?,
            IssueTitle::from_str("title")?,
            Some(IssueDue::from_str("2021-02-03T04:05:06Z")?),
        )?;
        let _ = issue.update(None, Instant::now())?;
        // TODO: assert
        Ok(())
    }

    #[test]
    fn truncate_events_test() {
        // TODO
    }
}
