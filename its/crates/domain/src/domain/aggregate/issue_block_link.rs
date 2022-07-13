mod error;
mod event;

use limited_date_time::Instant;

use crate::{domain::entity::IssueBlockLink, IssueBlockLinkId, IssueId, Version};
use crate::{IssueBlocked, IssueUnblocked};

pub use self::error::Error;
pub use self::event::IssueBlockLinkAggregateEvent;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueBlockLinkAggregate {
    events: Vec<IssueBlockLinkAggregateEvent>,
    issue_block_link: IssueBlockLink,
    version: Version,
}

impl IssueBlockLinkAggregate {
    pub fn from_events(events: &[IssueBlockLinkAggregateEvent]) -> Result<Self> {
        let first_event = match events.first() {
            Some(event) => match event {
                IssueBlockLinkAggregateEvent::Blocked(event) => Ok(event),
                IssueBlockLinkAggregateEvent::Unblocked(_) => Err(Error::InvalidEventSequence),
            },
            None => Err(Error::InvalidEventSequence),
        }?;
        let mut issue_block_link = Self::from_event(first_event)?;
        for event in events.iter().skip(1) {
            issue_block_link = issue_block_link.apply_event(event.clone())?;
        }
        Ok(issue_block_link.truncate_events())
    }

    pub fn new(at: Instant, issue_id: IssueId, blocked_issue_id: IssueId) -> Result<Self> {
        let id = IssueBlockLinkId::new(issue_id, blocked_issue_id).map_err(|_| Error::Block)?;
        let issue_block_link = IssueBlockLink::new(id.clone());
        let version = Version::from(1_u64);
        Ok(Self {
            events: vec![IssueBlockLinkAggregateEvent::Blocked(IssueBlocked {
                at,
                issue_block_link_id: id,
                version,
            })],
            issue_block_link,
            version,
        })
    }

    pub fn block(&self, at: Instant) -> Result<Self> {
        let event = IssueBlocked {
            at,
            issue_block_link_id: self.issue_block_link.id().clone(),
            version: self.next_version()?,
        };
        self.apply_event(event.into())
    }

    pub fn events(&self) -> &Vec<IssueBlockLinkAggregateEvent> {
        &self.events
    }

    pub fn id(&self) -> &IssueBlockLinkId {
        self.issue_block_link.id()
    }

    // query
    pub fn is_blocked(&self) -> bool {
        self.issue_block_link.is_blocked()
    }

    pub fn truncate_events(self) -> Self {
        Self {
            events: vec![],
            issue_block_link: self.issue_block_link,
            version: self.version,
        }
    }

    pub fn unblock(&self, at: Instant) -> Result<Self> {
        let event = IssueUnblocked {
            at,
            issue_block_link_id: self.issue_block_link.id().clone(),
            version: self.next_version()?,
        };
        self.apply_event(event.into())
    }

    fn apply_event(&self, event: IssueBlockLinkAggregateEvent) -> Result<Self> {
        let events = [self.events.as_slice(), &[event.clone()]].concat();
        let (issue_block_link_id, version) = event.key();
        if issue_block_link_id != self.issue_block_link.id() {
            return Err(Error::InvalidEventSequence);
        }
        if version != self.next_version()? {
            return Err(Error::InvalidEventSequence);
        }
        let issue_block_link = match event {
            IssueBlockLinkAggregateEvent::Blocked(_) => self.issue_block_link.block(),
            IssueBlockLinkAggregateEvent::Unblocked(_) => self.issue_block_link.unblock(),
        };
        Ok(Self {
            events,
            issue_block_link,
            version,
        })
    }

    fn from_event(event: &IssueBlocked) -> Result<Self> {
        Self::new(
            event.at(),
            event.issue_id().clone(),
            event.blocked_issue_id().clone(),
        )
        .map(Self::truncate_events)
    }

    fn next_version(&self) -> Result<Version> {
        self.version.next().ok_or(Error::NoNextVersion)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use limited_date_time::Instant;

    use crate::{IssueBlockLinkId, IssueBlocked, IssueId, IssueUnblocked, Version};

    use super::IssueBlockLinkAggregate;

    #[test]
    fn from_event_test() -> anyhow::Result<()> {
        let issue_block_link_id = IssueBlockLinkId::from_str("123 -> 456")?;
        let events = vec![
            IssueBlocked::from_trusted_data(
                Instant::now(),
                issue_block_link_id.clone(),
                Version::from(1_u64),
            )
            .into(),
            IssueUnblocked::from_trusted_data(
                Instant::now(),
                issue_block_link_id.clone(),
                Version::from(2_u64),
            )
            .into(),
        ];
        let created = IssueBlockLinkAggregate::from_events(&events)?;
        assert_eq!(created.id(), &issue_block_link_id);
        assert!(created.events().is_empty());
        Ok(())
    }

    #[test]
    fn id_test() -> anyhow::Result<()> {
        let issue_id = IssueId::from_str("123")?;
        let blocked_issue_id = IssueId::from_str("456")?;
        let created = IssueBlockLinkAggregate::new(
            Instant::now(),
            issue_id.clone(),
            blocked_issue_id.clone(),
        )?;
        let id = IssueBlockLinkId::new(issue_id, blocked_issue_id)?;
        assert_eq!(created.id(), &id);
        Ok(())
    }

    #[test]
    fn is_blocked_test() -> anyhow::Result<()> {
        let issue_id = IssueId::from_str("123")?;
        let blocked_issue_id = IssueId::from_str("456")?;
        let created = IssueBlockLinkAggregate::new(Instant::now(), issue_id, blocked_issue_id)?;
        assert!(created.is_blocked());
        let updated = created.unblock(Instant::now())?;
        assert!(!updated.is_blocked());
        Ok(())
    }

    #[test]
    fn truncate_events_test() -> anyhow::Result<()> {
        let issue_id = IssueId::from_str("123")?;
        let blocked_issue_id = IssueId::from_str("456")?;
        let created = IssueBlockLinkAggregate::new(Instant::now(), issue_id, blocked_issue_id)?;
        assert!(!created.events().is_empty());
        assert!(created.truncate_events().events().is_empty());
        Ok(())
    }
}
