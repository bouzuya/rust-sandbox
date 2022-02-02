mod issue_repository;

pub use self::issue_repository::*;
use async_trait::async_trait;
use domain::{
    aggregate::{
        IssueAggregate, IssueAggregateCommand, IssueAggregateCreateIssue, IssueAggregateError,
        IssueAggregateEvent, IssueAggregateFinishIssue, IssueAggregateUpdateIssue,
    },
    IssueCreatedV2, IssueDue, IssueFinished, IssueId, IssueNumber, IssueTitle, IssueUpdated,
};
use limited_date_time::Instant;
use thiserror::Error;

#[derive(Debug)]
pub enum IssueManagementContextCommand {
    CreateIssue(CreateIssue),
    FinishIssue(FinishIssue),
    UpdateIssue(UpdateIssue),
}

#[derive(Debug)]
pub struct CreateIssue {
    pub issue_title: IssueTitle,
    pub issue_due: Option<IssueDue>,
}

#[derive(Debug)]
pub struct FinishIssue {
    pub issue_id: IssueId,
}

#[derive(Debug)]
pub struct UpdateIssue {
    pub issue_id: IssueId,
    pub issue_due: Option<IssueDue>,
}

#[derive(Debug)]
pub enum IssueManagementContextEvent {
    IssueCreated(IssueCreatedV2),
    IssueFinished(IssueFinished),
    IssueUpdated(IssueUpdated),
}

#[derive(Debug, Error)]
pub enum IssueManagementContextError {
    #[error("IssueAggregate")]
    IssueAggregate(IssueAggregateError),
    #[error("Unknown")]
    Unknown,
}

#[async_trait]
pub trait IssueManagementContextUseCase: HasIssueRepository {
    async fn handle(
        &self,
        command: IssueManagementContextCommand,
    ) -> Result<IssueManagementContextEvent, IssueManagementContextError> {
        match command {
            IssueManagementContextCommand::CreateIssue(command) => {
                let event = self.create_issue_use_case(command).await?;
                Ok(IssueManagementContextEvent::IssueCreated(event))
            }
            IssueManagementContextCommand::FinishIssue(command) => {
                let event = self.finish_issue_use_case(command).await?;
                Ok(IssueManagementContextEvent::IssueFinished(event))
            }
            IssueManagementContextCommand::UpdateIssue(command) => {
                let event = self.update_issue_use_case(command).await?;
                Ok(IssueManagementContextEvent::IssueUpdated(event))
            }
        }
    }

    async fn create_issue_use_case(
        &self,
        command: CreateIssue,
    ) -> Result<IssueCreatedV2, IssueManagementContextError> {
        // io
        let issue_number = self
            .issue_repository()
            .last_created()
            .await
            .map_err(|_| IssueManagementContextError::Unknown)?
            .map(|issue| issue.id().issue_number())
            .unwrap_or_else(IssueNumber::start_number)
            .next_number();
        let at = Instant::now();

        // pure
        let (_, event) =
            IssueAggregate::transaction(IssueAggregateCommand::Create(IssueAggregateCreateIssue {
                issue_number,
                issue_title: command.issue_title,
                issue_due: command.issue_due,
                at,
            }))
            .map_err(IssueManagementContextError::IssueAggregate)?;

        // io
        self.issue_repository()
            .save(event.clone())
            .await
            .map_err(|_| IssueManagementContextError::Unknown)?;

        if let IssueAggregateEvent::CreatedV2(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }

    async fn finish_issue_use_case(
        &self,
        command: FinishIssue,
    ) -> Result<IssueFinished, IssueManagementContextError> {
        // io
        let issue = self
            .issue_repository()
            .find_by_id(&command.issue_id)
            .await
            .map_err(|_| IssueManagementContextError::Unknown)?;
        // TODO: fix error
        let issue = issue.ok_or(IssueManagementContextError::Unknown)?;
        let at = Instant::now();

        // pure
        let (_, event) =
            IssueAggregate::transaction(IssueAggregateCommand::Finish(IssueAggregateFinishIssue {
                issue,
                at,
            }))
            .map_err(IssueManagementContextError::IssueAggregate)?;

        // io
        self.issue_repository()
            .save(event.clone())
            .await
            .map_err(|_| IssueManagementContextError::Unknown)?;

        if let IssueAggregateEvent::Finished(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }

    async fn update_issue_use_case(
        &self,
        command: UpdateIssue,
    ) -> Result<IssueUpdated, IssueManagementContextError> {
        // io
        let issue = self
            .issue_repository()
            .find_by_id(&command.issue_id)
            .await
            .map_err(|_| IssueManagementContextError::Unknown)?;
        // TODO: fix error
        let issue = issue.ok_or(IssueManagementContextError::Unknown)?;
        let issue_due = command.issue_due;
        let at = Instant::now();

        // pure
        let (_, event) =
            IssueAggregate::transaction(IssueAggregateCommand::Update(IssueAggregateUpdateIssue {
                issue,
                issue_due,
                at,
            }))
            .map_err(IssueManagementContextError::IssueAggregate)?;

        // io
        self.issue_repository()
            .save(event.clone())
            .await
            .map_err(|_| IssueManagementContextError::Unknown)?;

        if let IssueAggregateEvent::Updated(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }
}

impl<T: HasIssueRepository> IssueManagementContextUseCase for T {}

pub trait HasIssueManagementContextUseCase {
    type IssueManagementContextUseCase: IssueManagementContextUseCase;
    fn issue_management_context_use_case(&self) -> &Self::IssueManagementContextUseCase;
}
