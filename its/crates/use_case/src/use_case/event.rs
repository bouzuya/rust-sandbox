use domain::DomainEvent;

#[derive(Clone, Debug)]
pub struct IssueManagementContextEvent(DomainEvent);

impl From<DomainEvent> for IssueManagementContextEvent {
    fn from(event: DomainEvent) -> Self {
        IssueManagementContextEvent(event)
    }
}

impl From<IssueManagementContextEvent> for DomainEvent {
    fn from(event: IssueManagementContextEvent) -> Self {
        event.0
    }
}
