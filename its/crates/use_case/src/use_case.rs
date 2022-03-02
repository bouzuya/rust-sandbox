mod command;
mod event;
mod issue_block_link_repository;
mod issue_repository;

pub use self::command::*;
pub use self::event::IssueManagementContextEvent;
pub use self::issue_block_link_repository::*;
pub use self::issue_block_link_repository::*;
pub use self::issue_repository::*;
use async_trait::async_trait;
use domain::aggregate::IssueBlockLinkAggregateError;
use domain::aggregate::IssueBlockLinkAggregateEvent;
use domain::DomainEvent;
use domain::IssueBlocked;
use domain::{
    aggregate::{
        IssueAggregate, IssueAggregateCommand, IssueAggregateCreateIssue, IssueAggregateError,
        IssueAggregateEvent, IssueAggregateFinishIssue, IssueAggregateUpdateIssue,
    },
    IssueCreatedV2, IssueDue, IssueFinished, IssueId, IssueNumber, IssueTitle, IssueUpdated,
};
use limited_date_time::Instant;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssueManagementContextError {
    #[error("IssueAggregate")]
    IssueAggregate(#[from] IssueAggregateError),
    #[error("IssueBlockLinkAggregate")]
    IssueBlockLinkAggregate(#[from] IssueBlockLinkAggregateError),
    #[error("IssueBlockLinkRepository")]
    IssueBlockLinkRepository(#[from] IssueBlockLinkRepositoryError),
    #[error("IssueNotFound")]
    IssueNotFound(IssueId),
    #[error("IssueRepository")]
    IssueRepository(#[from] IssueRepositoryError),
    #[error("Unknown")]
    Unknown,
}

#[async_trait]
pub trait IssueManagementContextUseCase: HasIssueRepository + HasIssueBlockLinkRepository {
    async fn handle(
        &self,
        command: IssueManagementContextCommand,
    ) -> Result<IssueManagementContextEvent, IssueManagementContextError> {
        match command {
            IssueManagementContextCommand::BlockIssue(command) => {
                let event = self.handle_block_issue(command).await?;
                Ok(IssueManagementContextEvent::from(
                    DomainEvent::IssueBlockLink(IssueBlockLinkAggregateEvent::from(event)),
                ))
            }
            IssueManagementContextCommand::CreateIssue(command) => {
                let event = self.handle_create_issue(command).await?;
                Ok(IssueManagementContextEvent::from(DomainEvent::Issue(
                    IssueAggregateEvent::from(event),
                )))
            }
            IssueManagementContextCommand::FinishIssue(command) => {
                let event = self.handle_finish_issue(command).await?;
                Ok(IssueManagementContextEvent::from(DomainEvent::Issue(
                    IssueAggregateEvent::from(event),
                )))
            }
            IssueManagementContextCommand::UpdateIssue(command) => {
                let event = self.handle_update_issue(command).await?;
                Ok(IssueManagementContextEvent::from(DomainEvent::Issue(
                    IssueAggregateEvent::from(event),
                )))
            }
        }
    }

    fn block_issue(&self, issue_id: IssueId, blocked_issue_id: IssueId) -> BlockIssue {
        BlockIssue {
            issue_id,
            blocked_issue_id,
        }
    }

    fn create_issue(&self, issue_title: IssueTitle, issue_due: Option<IssueDue>) -> CreateIssue {
        CreateIssue {
            issue_title,
            issue_due,
        }
    }

    fn finish_issue(&self, issue_id: IssueId) -> FinishIssue {
        FinishIssue { issue_id }
    }

    fn update_issue(&self, issue_id: IssueId, issue_due: Option<IssueDue>) -> UpdateIssue {
        UpdateIssue {
            issue_id,
            issue_due,
        }
    }

    async fn handle_block_issue(
        &self,
        BlockIssue {
            issue_id,
            blocked_issue_id,
        }: BlockIssue,
    ) -> Result<IssueBlocked, IssueManagementContextError> {
        // io
        let issue = self
            .issue_repository()
            .find_by_id(&issue_id)
            .await?
            .ok_or(IssueManagementContextError::IssueNotFound(issue_id))?;
        let blocked_issue = self
            .issue_repository()
            .find_by_id(&blocked_issue_id)
            .await?
            .ok_or(IssueManagementContextError::IssueNotFound(blocked_issue_id))?;

        // pure
        let (_, event) = issue.block(blocked_issue)?;

        // io
        self.issue_block_link_repository().save(event).await?;

        todo!()
    }

    async fn handle_create_issue(
        &self,
        command: CreateIssue,
    ) -> Result<IssueCreatedV2, IssueManagementContextError> {
        // io
        let issue_number = self
            .issue_repository()
            .last_created()
            .await?
            .map(|issue| issue.id().issue_number().next_number())
            .unwrap_or_else(IssueNumber::start_number);
        let at = Instant::now();

        // pure
        let (_, event) = IssueAggregate::transaction(IssueAggregateCommand::Create(
            IssueAggregateCreateIssue {
                issue_number,
                issue_title: command.issue_title,
                issue_due: command.issue_due,
                at,
            },
        ))?;

        // io
        self.issue_repository().save(event.clone()).await?;

        if let IssueAggregateEvent::CreatedV2(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }

    async fn handle_finish_issue(
        &self,
        command: FinishIssue,
    ) -> Result<IssueFinished, IssueManagementContextError> {
        // io
        let issue = self
            .issue_repository()
            .find_by_id(&command.issue_id)
            .await?
            .ok_or(IssueManagementContextError::IssueNotFound(command.issue_id))?;
        let at = Instant::now();

        // pure
        let (_, event) = IssueAggregate::transaction(IssueAggregateCommand::Finish(
            IssueAggregateFinishIssue { issue, at },
        ))?;

        // io
        self.issue_repository().save(event.clone()).await?;

        if let IssueAggregateEvent::Finished(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }

    async fn handle_update_issue(
        &self,
        command: UpdateIssue,
    ) -> Result<IssueUpdated, IssueManagementContextError> {
        // io
        let issue = self
            .issue_repository()
            .find_by_id(&command.issue_id)
            .await?;
        // TODO: fix error
        let issue = issue.ok_or(IssueManagementContextError::Unknown)?;
        let issue_due = command.issue_due;
        let at = Instant::now();

        // pure
        let (_, event) = IssueAggregate::transaction(IssueAggregateCommand::Update(
            IssueAggregateUpdateIssue {
                issue,
                issue_due,
                at,
            },
        ))?;

        // io
        self.issue_repository().save(event.clone()).await?;

        if let IssueAggregateEvent::Updated(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }
}

impl<T: HasIssueRepository + HasIssueBlockLinkRepository> IssueManagementContextUseCase for T {}

pub trait HasIssueManagementContextUseCase {
    type IssueManagementContextUseCase: IssueManagementContextUseCase;
    fn issue_management_context_use_case(&self) -> &Self::IssueManagementContextUseCase;
}
