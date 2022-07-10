use domain::{IssueId, IssueResolution};
use limited_date_time::Instant;

use crate::{
    HasIssueRepository, IssueManagementContextEvent, IssueRepository, IssueRepositoryError,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("issue aggregate {0}")]
    IssueAggregate(#[from] domain::aggregate::issue::Error),
    #[error("issue not found {0}")]
    IssueNotFound(IssueId),
    #[error("issue repository {0}")]
    IssueRepository(#[from] IssueRepositoryError),
}

#[derive(Debug, Eq, PartialEq)]
pub struct FinishIssue {
    pub issue_id: IssueId,
    pub resolution: Option<IssueResolution>,
}

pub async fn finish_issue<C: HasIssueRepository + ?Sized>(
    context: &C,
    command: FinishIssue,
) -> Result<IssueManagementContextEvent, Error> {
    // io
    let issue = context
        .issue_repository()
        .find_by_id(&command.issue_id)
        .await?
        .ok_or(Error::IssueNotFound(command.issue_id))?;
    let resolution = command.resolution;
    let at = Instant::now();

    // pure
    let updated = issue.finish(resolution, at)?;

    // io
    context.issue_repository().save(&updated).await?;

    let issue_id = updated
        .events()
        .iter()
        .next()
        .map(|event| event.issue_id().to_owned())
        .expect("invalid event seq");
    Ok(IssueManagementContextEvent::IssueUpdated { issue_id })
}
