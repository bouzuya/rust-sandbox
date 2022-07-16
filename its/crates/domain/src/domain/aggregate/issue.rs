mod entity;
mod error;
mod event;

use limited_date_time::Instant;

use self::entity::issue::Issue;
pub use self::error::*;
pub use self::event::*;
use crate::IssueCreatedV2;
use crate::IssueDescription;
use crate::IssueDescriptionUpdated;
use crate::IssueDue;
use crate::IssueNumber;
use crate::IssueResolution;
use crate::IssueStatus;
use crate::IssueTitle;
use crate::IssueTitleUpdated;
use crate::IssueUpdated;
use crate::{domain::event::IssueFinished, IssueId, Version};

pub use self::error::Error;
use super::IssueBlockLinkAggregate;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueAggregate {
    events: Vec<IssueAggregateEvent>,
    issue: Issue,
    version: Version,
}

impl IssueAggregate {
    pub fn from_events(events: &[IssueAggregateEvent]) -> Result<Self> {
        let first_event = match events.first() {
            Some(event) => match event {
                IssueAggregateEvent::Created(event) => Ok(IssueCreatedV2::from(event.clone())),
                IssueAggregateEvent::CreatedV2(event) => Ok(event.clone()),
                IssueAggregateEvent::DescriptionUpdated(_) => Err(Error::InvalidEventSequence),
                IssueAggregateEvent::Finished(_) => Err(Error::InvalidEventSequence),
                IssueAggregateEvent::TitleUpdated(_) => Err(Error::InvalidEventSequence),
                IssueAggregateEvent::Updated(_) => Err(Error::InvalidEventSequence),
            },
            None => Err(Error::InvalidEventSequence),
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
                    return Err(Error::InvalidEventSequence);
                }
                IssueAggregateEvent::CreatedV2(_) => {
                    return Err(Error::InvalidEventSequence);
                }
                IssueAggregateEvent::DescriptionUpdated(IssueDescriptionUpdated {
                    issue_id,
                    issue_description,
                    version,
                    ..
                }) => {
                    if issue.issue.id() != issue_id {
                        return Err(Error::InvalidEventSequence);
                    }
                    if issue.version.next() != Some(*version) {
                        return Err(Error::InvalidEventSequence);
                    }

                    issue = IssueAggregate {
                        events: vec![],
                        issue: issue.issue.change_description(issue_description.clone()),
                        version: *version,
                    }
                }
                IssueAggregateEvent::Finished(IssueFinished {
                    at: _,
                    issue_id,
                    resolution,
                    version,
                }) => {
                    if issue.issue.id() != issue_id {
                        return Err(Error::InvalidEventSequence);
                    }
                    if issue.version.next() != Some(*version) {
                        return Err(Error::InvalidEventSequence);
                    }

                    issue = IssueAggregate {
                        events: vec![],
                        issue: issue
                            .issue
                            .finish(resolution.clone())
                            .map_err(|_| Error::InvalidEventSequence)?,
                        version: *version,
                    }
                }
                IssueAggregateEvent::TitleUpdated(IssueTitleUpdated {
                    at: _,
                    issue_id,
                    issue_title,
                    version,
                }) => {
                    if issue.issue.id() != issue_id {
                        return Err(Error::InvalidEventSequence);
                    }
                    if issue.version.next() != Some(*version) {
                        return Err(Error::InvalidEventSequence);
                    }

                    issue = IssueAggregate {
                        events: vec![],
                        issue: issue.issue.change_title(issue_title.clone()),
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
                        return Err(Error::InvalidEventSequence);
                    }
                    if issue.version.next() != Some(*version) {
                        return Err(Error::InvalidEventSequence);
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
        issue_description: IssueDescription,
    ) -> Result<Self> {
        let issue_id = IssueId::new(issue_number);
        let issue = Issue::new(
            issue_id.clone(),
            issue_title.clone(),
            issue_due,
            issue_description.clone(),
        );
        let version = Version::from(1_u64);
        let event = IssueCreatedV2 {
            at,
            issue_id,
            issue_title,
            issue_due,
            issue_description,
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

    pub fn finish(&self, resolution: Option<IssueResolution>, at: Instant) -> Result<Self> {
        let updated_issue = self
            .issue
            .finish(resolution.clone())
            .map_err(|_| Error::Unknown)?;
        let updated_version = self.version.next().ok_or(Error::Unknown)?;
        let event = IssueFinished {
            at,
            issue_id: self.id().clone(),
            resolution,
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

    pub fn update(&self, issue_due: Option<IssueDue>, at: Instant) -> Result<Self> {
        let updated_issue = self.issue.change_due(issue_due);
        let updated_version = self.version.next().ok_or(Error::Unknown)?;
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

    pub fn update_description(
        &self,
        issue_description: IssueDescription,
        at: Instant,
    ) -> Result<Self> {
        let updated_issue = self.issue.change_description(issue_description);
        let updated_version = self.version.next().ok_or(Error::Unknown)?;
        let event = IssueDescriptionUpdated {
            at,
            issue_id: self.id().clone(),
            issue_description: updated_issue.description().clone(),
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

    pub fn update_title(&self, issue_title: IssueTitle, at: Instant) -> Result<Self> {
        let updated_issue = self.issue.change_title(issue_title);
        let updated_version = self.version.next().ok_or(Error::Unknown)?;
        let event = IssueTitleUpdated {
            at,
            issue_id: self.id().clone(),
            issue_title: updated_issue.title().clone(),
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

    pub fn description(&self) -> &IssueDescription {
        self.issue.description()
    }

    pub fn events(&self) -> &Vec<IssueAggregateEvent> {
        &self.events
    }

    pub fn id(&self) -> &IssueId {
        self.issue.id()
    }

    pub fn resolution(&self) -> Option<&IssueResolution> {
        self.issue.resolution()
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

    pub fn version(&self) -> Version {
        self.version
    }

    // factory for IssueBlockLinkAggregate
    pub fn block(
        &self,
        blocked_issue: IssueAggregate,
        at: Instant,
    ) -> Result<IssueBlockLinkAggregate, super::issue_block_link::Error> {
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
        let issue_description = IssueDescription::from_str("desc1")?;
        let issue = IssueAggregate::new(
            Instant::now(),
            issue_number,
            issue_title.clone(),
            issue_due,
            issue_description.clone(),
        )?;
        assert_eq!(issue.id().issue_number(), issue_number);
        assert_eq!(issue.title(), &issue_title);
        assert_eq!(issue.due(), issue_due);
        assert_eq!(issue.description(), &issue_description);
        Ok(())
    }

    #[test]
    fn finish_test() -> anyhow::Result<()> {
        let issue = IssueAggregate::new(
            Instant::now(),
            IssueNumber::from_str("123")?,
            IssueTitle::from_str("title")?,
            Some(IssueDue::from_str("2021-02-03T04:05:06Z")?),
            IssueDescription::from_str("desc1")?,
        )?;
        let resolution = IssueResolution::from_str("Duplicate")?;
        let _ = issue.finish(Some(resolution), Instant::now())?;
        // TODO: assert
        Ok(())
    }

    #[test]
    fn update_test() -> anyhow::Result<()> {
        let issue = IssueAggregate::new(
            Instant::now(),
            IssueNumber::from_str("123")?,
            IssueTitle::from_str("title")?,
            Some(IssueDue::from_str("2021-02-03T04:05:06Z")?),
            IssueDescription::from_str("desc1")?,
        )?;
        let _ = issue.update(None, Instant::now())?;
        // TODO: assert
        Ok(())
    }

    #[test]
    fn update_description_test() -> anyhow::Result<()> {
        let issue = IssueAggregate::new(
            Instant::now(),
            IssueNumber::from_str("123")?,
            IssueTitle::from_str("title")?,
            Some(IssueDue::from_str("2021-02-03T04:05:06Z")?),
            IssueDescription::from_str("desc1")?,
        )?;
        let description = IssueDescription::from_str("desc2")?;
        let updated = issue.update_description(description.clone(), Instant::now())?;
        assert_eq!(updated.description(), &description);
        Ok(())
    }

    #[test]
    fn update_title_test() -> anyhow::Result<()> {
        let issue = IssueAggregate::new(
            Instant::now(),
            IssueNumber::from_str("123")?,
            IssueTitle::from_str("title")?,
            Some(IssueDue::from_str("2021-02-03T04:05:06Z")?),
            IssueDescription::from_str("desc1")?,
        )?;
        let title = IssueTitle::from_str("title2")?;
        let updated = issue.update_title(title.clone(), Instant::now())?;
        assert_eq!(updated.title(), &title);
        Ok(())
    }

    #[test]
    fn truncate_events_test() {
        // TODO
    }
}
