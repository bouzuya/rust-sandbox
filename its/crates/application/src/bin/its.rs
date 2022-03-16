use std::{path::PathBuf, str::FromStr};

use adapter_sqlite::{
    SqliteConnectionPool, SqliteIssueBlockLinkRepository, SqliteIssueRepository, SqliteQueryHandler,
};
use domain::{IssueBlockLinkId, IssueDue, IssueId, IssueTitle};
use use_case::{
    HasIssueBlockLinkRepository, HasIssueManagementContextUseCase, HasIssueRepository,
    IssueManagementContextUseCase,
};
use xdg::BaseDirectories;

struct App {
    issue_block_link_repository: SqliteIssueBlockLinkRepository,
    issue_repository: SqliteIssueRepository,
    query_handler: SqliteQueryHandler,
}

impl App {
    async fn new() -> anyhow::Result<Self> {
        let data_dir = Self::state_dir()?;
        let query_handler = SqliteQueryHandler::new(data_dir.as_path()).await?;
        let connection_pool = SqliteConnectionPool::new(data_dir).await?;
        let issue_block_link_repository =
            SqliteIssueBlockLinkRepository::new(connection_pool.clone()).await?;
        let issue_repository =
            SqliteIssueRepository::new(connection_pool, query_handler.clone()).await?;
        Ok(Self {
            issue_block_link_repository,
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

impl HasIssueBlockLinkRepository for App {
    type IssueBlockLinkRepository = SqliteIssueBlockLinkRepository;

    fn issue_block_link_repository(&self) -> &Self::IssueBlockLinkRepository {
        &self.issue_block_link_repository
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

#[argopt::subcmd(name = "issue-block")]
fn issue_block(
    #[opt(name = "issue-id")] issue_id: String,
    #[opt(name = "blocked-issue-id")] blocked_issue_id: String,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new().await?;
            let use_case = app.issue_management_context_use_case();
            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let blocked_issue_id = IssueId::from_str(blocked_issue_id.as_str())?;
            let command = use_case.block_issue(issue_id, blocked_issue_id).into();
            let event = use_case.handle(command).await?;
            println!("issue blocked : {:?}", event);
            Ok(())
        })
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
            let use_case = app.issue_management_context_use_case();
            let issue_title = IssueTitle::try_from(title.unwrap_or_default())?;
            let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
            let command = use_case.create_issue(issue_title, issue_due).into();
            let event = use_case.handle(command).await?;
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
            let use_case = app.issue_management_context_use_case();
            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let command = use_case.finish_issue(issue_id).into();
            let event = use_case.handle(command).await?;
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

#[argopt::subcmd(name = "issue-unblock")]
fn issue_unblock(
    #[opt(name = "issue-id")] issue_id: String,
    #[opt(name = "blocked-issue-id")] blocked_issue_id: String,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new().await?;
            let use_case = app.issue_management_context_use_case();
            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let blocked_issue_id = IssueId::from_str(blocked_issue_id.as_str())?;
            let issue_block_link_id = IssueBlockLinkId::new(issue_id, blocked_issue_id)?;
            let command = use_case.unblock_issue(issue_block_link_id).into();
            let event = use_case.handle(command).await?;
            println!("issue unblocked : {:?}", event);
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
            let use_case = app.issue_management_context_use_case();
            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
            let command = use_case.update_issue(issue_id, issue_due).into();
            let event = app
                .issue_management_context_use_case()
                .handle(command)
                .await?;
            println!("issue updated : {:?}", event);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-view")]
fn issue_view(issue_id: String) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new().await?;
            let issue_id = IssueId::from_str(issue_id.as_str())?;

            let issue = app.query_handler.issue_view(&issue_id).await?;
            println!("{}", serde_json::to_string(&issue)?);
            Ok(())
        })
}

#[argopt::cmd_group(commands = [
    issue_block,
    issue_create,
    issue_finish,
    issue_list,
    issue_unblock,
    issue_update,
    issue_view
])]
fn main() -> anyhow::Result<()> {}
