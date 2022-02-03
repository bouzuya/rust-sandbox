use crate::{
    domain::event::{IssueCreated, IssueCreatedV2, IssueFinished},
    IssueId, IssueUpdated, Version,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueAggregateEvent {
    Created(IssueCreated),
    CreatedV2(IssueCreatedV2),
    Finished(IssueFinished),
    Updated(IssueUpdated),
}

impl IssueAggregateEvent {
    pub fn issue_id(&self) -> &IssueId {
        match self {
            IssueAggregateEvent::Created(IssueCreated { issue_id, .. }) => issue_id,
            IssueAggregateEvent::CreatedV2(IssueCreatedV2 { issue_id, .. }) => issue_id,
            IssueAggregateEvent::Finished(IssueFinished { issue_id, .. }) => issue_id,
            IssueAggregateEvent::Updated(IssueUpdated { issue_id, .. }) => issue_id,
        }
    }

    pub fn version(&self) -> Version {
        match self {
            IssueAggregateEvent::Created(IssueCreated { version, .. }) => *version,
            IssueAggregateEvent::CreatedV2(IssueCreatedV2 { version, .. }) => *version,
            IssueAggregateEvent::Finished(IssueFinished { version, .. }) => *version,
            IssueAggregateEvent::Updated(IssueUpdated { version, .. }) => *version,
        }
    }
}
