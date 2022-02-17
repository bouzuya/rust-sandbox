use std::{path::PathBuf, str::FromStr};

use adapter_sqlite::{SqliteIssueRepository, SqliteQueryHandler};
use domain::{IssueDue, IssueId, IssueTitle};
use use_case::{
    CreateIssue, FinishIssue, HasIssueManagementContextUseCase, HasIssueRepository,
    IssueManagementContextCommand, IssueManagementContextUseCase, UpdateIssue,
};
use xdg::BaseDirectories;

struct App {
    issue_repository: SqliteIssueRepository,
    query_handler: SqliteQueryHandler,
}

impl App {
    async fn new() -> anyhow::Result<Self> {
        let data_dir = Self::state_dir()?;
        let query_handler = SqliteQueryHandler::new(data_dir.as_path()).await?;
        let issue_repository = SqliteIssueRepository::new(data_dir).await?;
        Ok(Self {
            issue_repository,
            query_handler,
        })
    }

    fn state_dir() -> anyhow::Result<PathBuf> {
        // $XDG_STATE_HOME/$prefix
        // $HOME/.local/state/$prefix
        let prefix = "net.bouzuya.rust-sandbox.its";
        Ok(BaseDirectories::with_prefix(prefix)?.get_state_home())
    }
}

impl HasIssueRepository for App {
    type IssueRepository = SqliteIssueRepository;

    fn issue_repository(&self) -> &Self::IssueRepository {
        &self.issue_repository
    }
}

impl HasIssueManagementContextUseCase for App {
    type IssueManagementContextUseCase = App;

    fn issue_management_context_use_case(&self) -> &Self::IssueManagementContextUseCase {
        self
    }
}

#[argopt::subcmd(name = "issue-create")]
fn issue_create(
    #[opt(long = "title")] title: Option<String>,
    #[opt(long = "due")] due: Option<String>,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new().await?;

            let issue_title = IssueTitle::try_from(title.unwrap_or_default())?;
            let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
            let command = IssueManagementContextCommand::CreateIssue(CreateIssue {
                issue_title,
                issue_due,
            });
            let event = app
                .issue_management_context_use_case()
                .handle(command)
                .await?;
            println!("issue created : {:?}", event);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-finish")]
fn issue_finish(issue_id: String) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new().await?;

            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let command = IssueManagementContextCommand::FinishIssue(FinishIssue { issue_id });
            let event = app
                .issue_management_context_use_case()
                .handle(command)
                .await?;
            println!("issue finished : {:?}", event);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-list")]
fn issue_list() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new().await?;

            let issues = app.query_handler.issue_list().await?;
            println!("{}", serde_json::to_string(&issues)?);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-update")]
fn issue_update(issue_id: String, #[opt(long = "due")] due: Option<String>) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new().await?;

            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
            let command = IssueManagementContextCommand::UpdateIssue(UpdateIssue {
                issue_id,
                issue_due,
            });
            let event = app
                .issue_management_context_use_case()
                .handle(command)
                .await?;
            println!("issue updated : {:?}", event);
            Ok(())
        })
}

#[argopt::cmd_group(commands = [issue_create, issue_finish, issue_list, issue_update])]
fn main() -> anyhow::Result<()> {}
