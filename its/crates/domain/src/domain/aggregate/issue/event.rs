use crate::{
    domain::event::{IssueCreated, IssueCreatedV2, IssueFinished},
    IssueDescriptionUpdated, IssueId, IssueTitleUpdated, IssueUpdated, Version,
};

macro_rules! impl_from_ty_for_issue_aggregate_event {
    ($event_name:ty, $variant:expr) => {
        impl From<$event_name> for IssueAggregateEvent {
            fn from(event: $event_name) -> Self {
                $variant(event)
            }
        }
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueAggregateEvent {
    Created(IssueCreated),
    CreatedV2(IssueCreatedV2),
    DescriptionUpdated(IssueDescriptionUpdated),
    Finished(IssueFinished),
    Updated(IssueUpdated),
    TitleUpdated(IssueTitleUpdated),
}

impl_from_ty_for_issue_aggregate_event!(IssueCreated, Self::Created);
impl_from_ty_for_issue_aggregate_event!(IssueCreatedV2, Self::CreatedV2);
impl_from_ty_for_issue_aggregate_event!(IssueDescriptionUpdated, Self::DescriptionUpdated);
impl_from_ty_for_issue_aggregate_event!(IssueFinished, Self::Finished);
impl_from_ty_for_issue_aggregate_event!(IssueTitleUpdated, Self::TitleUpdated);
impl_from_ty_for_issue_aggregate_event!(IssueUpdated, Self::Updated);

impl IssueAggregateEvent {
    pub fn issue_id(&self) -> &IssueId {
        match self {
            IssueAggregateEvent::Created(IssueCreated { issue_id, .. }) => issue_id,
            IssueAggregateEvent::CreatedV2(IssueCreatedV2 { issue_id, .. }) => issue_id,
            IssueAggregateEvent::DescriptionUpdated(IssueDescriptionUpdated {
                issue_id, ..
            }) => issue_id,
            IssueAggregateEvent::Finished(IssueFinished { issue_id, .. }) => issue_id,
            IssueAggregateEvent::TitleUpdated(IssueTitleUpdated { issue_id, .. }) => issue_id,
            IssueAggregateEvent::Updated(IssueUpdated { issue_id, .. }) => issue_id,
        }
    }

    pub fn version(&self) -> Version {
        match self {
            IssueAggregateEvent::Created(IssueCreated { version, .. }) => *version,
            IssueAggregateEvent::CreatedV2(IssueCreatedV2 { version, .. }) => *version,
            IssueAggregateEvent::DescriptionUpdated(IssueDescriptionUpdated {
                version, ..
            }) => *version,
            IssueAggregateEvent::Finished(IssueFinished { version, .. }) => *version,
            IssueAggregateEvent::TitleUpdated(IssueTitleUpdated { version, .. }) => *version,
            IssueAggregateEvent::Updated(IssueUpdated { version, .. }) => *version,
        }
    }
}
