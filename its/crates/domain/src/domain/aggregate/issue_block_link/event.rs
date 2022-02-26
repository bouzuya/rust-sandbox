use crate::IssueBlocked;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueBlockLinkAggregateEvent {
    Blocked(IssueBlocked),
    // TODO: Id
    Unblocked,
}

impl From<IssueBlocked> for IssueBlockLinkAggregateEvent {
    fn from(event: IssueBlocked) -> Self {
        Self::Blocked(event)
    }
}
