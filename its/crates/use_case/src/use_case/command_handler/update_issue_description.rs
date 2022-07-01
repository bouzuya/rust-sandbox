use domain::{IssueDescription, IssueId};

use crate::{HasIssueRepository, IssueManagementContextEvent, IssueRepositoryError};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("issue aggregate {0}")]
    IssueAggregate(#[from] domain::aggregate::IssueAggregateError),
    #[error("issue not found {0}")]
    IssueNotFound(IssueId),
    #[error("issue repository {0}")]
    IssueRepository(#[from] IssueRepositoryError),
}

#[derive(Debug)]
pub struct UpdateIssueDescription {
    pub issue_id: IssueId,
    pub issue_description: IssueDescription,
}

pub async fn update_issue_description<C: HasIssueRepository + ?Sized>(
    _context: &C,
    _command: UpdateIssueDescription,
) -> Result<Vec<IssueManagementContextEvent>, Error> {
    todo!()
}
