use std::{
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use adapter_sqlite::{
    RdbConnectionPool, SqliteIssueBlockLinkRepository, SqliteIssueRepository, SqliteQueryHandler,
};
use anyhow::Context;
use clap::{Parser, Subcommand};
use domain::{DomainEvent, IssueBlockLinkId, IssueDue, IssueId, IssueTitle};
use use_case::{
    HasIssueBlockLinkRepository, HasIssueManagementContextUseCase, HasIssueRepository,
    IssueBlockLinkRepository, IssueManagementContextEvent, IssueManagementContextUseCase,
    IssueRepository,
};
use xdg::BaseDirectories;

struct App {
    issue_block_link_repository: SqliteIssueBlockLinkRepository,
    issue_repository: SqliteIssueRepository,
    query_handler: SqliteQueryHandler,
}

impl App {
    async fn new(
        command_database_connection_uri: Option<String>,
        query_database_connection_uri: Option<String>,
    ) -> anyhow::Result<Self> {
        let data_dir = Self::state_dir()?;
        if !data_dir.exists() {
            fs::create_dir_all(data_dir.as_path())?;
        }
        let new_connection_uri = |path: PathBuf| -> anyhow::Result<String> {
            Ok(format!(
                "sqlite:{}?mode=rwc",
                path.to_str().context("path is not utf-8")?
            ))
        };
        let command_connection_uri = match command_database_connection_uri {
            Some(s) => s,
            None => new_connection_uri(data_dir.join("command.sqlite"))?,
        };
        let query_connection_uri = match query_database_connection_uri {
            Some(s) => s,
            None => new_connection_uri(data_dir.join("query.sqlite"))?,
        };
        let connection_pool = RdbConnectionPool::new(&command_connection_uri).await?;
        let issue_block_link_repository =
            SqliteIssueBlockLinkRepository::new(connection_pool.clone()).await?;
        let issue_repository = SqliteIssueRepository::new(connection_pool.clone()).await?;
        let query_handler = SqliteQueryHandler::new(
            &query_connection_uri,
            Arc::new(Mutex::new(issue_repository)),
        )
        .await?;
        let issue_repository = SqliteIssueRepository::new(connection_pool).await?;
        Ok(Self {
            issue_block_link_repository,
            issue_repository,
            query_handler,
        })
    }

    // TODO: remove
    async fn update_query_db(&self, event: IssueManagementContextEvent) -> anyhow::Result<()> {
        if let DomainEvent::Issue(event) = DomainEvent::from(event.clone()) {
            if let Some(issue) = self.issue_repository().find_by_id(event.issue_id()).await? {
                self.query_handler.save_issue(issue).await?;
                // TODO: update query.issue_block_links.issue_title
                // TODO: update query.issue_block_links.blocked_issue_title
            }
        }
        if let DomainEvent::IssueBlockLink(event) = DomainEvent::from(event.clone()) {
            if let Some(issue_block_link) = self
                .issue_block_link_repository()
                .find_by_id(event.key().0)
                .await?
            {
                self.query_handler
                    .save_issue_block_link(issue_block_link)
                    .await?;
            }
        }
        Ok(())
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
    #[opt(long)] command_database_connection_uri: Option<String>,
    #[opt(long)] query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new(
                command_database_connection_uri,
                query_database_connection_uri,
            )
            .await?;
            let use_case = app.issue_management_context_use_case();
            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let blocked_issue_id = IssueId::from_str(blocked_issue_id.as_str())?;
            let command = use_case.block_issue(issue_id, blocked_issue_id).into();
            let events = use_case.handle(command).await?;
            // FIXME:
            app.update_query_db(events.first().unwrap().clone()).await?;
            println!("issue blocked : {:?}", events);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-create")]
fn issue_create(
    #[opt(long = "title")] title: Option<String>,
    #[opt(long = "due")] due: Option<String>,
    #[opt(long)] command_database_connection_uri: Option<String>,
    #[opt(long)] query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new(
                command_database_connection_uri,
                query_database_connection_uri,
            )
            .await?;
            let use_case = app.issue_management_context_use_case();
            let issue_title = IssueTitle::try_from(title.unwrap_or_default())?;
            let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
            let command = use_case.create_issue(issue_title, issue_due).into();
            let events = use_case.handle(command).await?;
            // FIXME:
            app.update_query_db(events.first().unwrap().clone()).await?;
            println!("issue created : {:?}", events);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-finish")]
fn issue_finish(
    issue_id: String,
    #[opt(long)] command_database_connection_uri: Option<String>,
    #[opt(long)] query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new(
                command_database_connection_uri,
                query_database_connection_uri,
            )
            .await?;
            let use_case = app.issue_management_context_use_case();
            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let command = use_case.finish_issue(issue_id).into();
            let events = use_case.handle(command).await?;
            // FIXME:
            app.update_query_db(events.first().unwrap().clone()).await?;
            println!("issue finished : {:?}", events);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-list")]
fn issue_list(
    #[opt(long)] command_database_connection_uri: Option<String>,
    #[opt(long)] query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new(
                command_database_connection_uri,
                query_database_connection_uri,
            )
            .await?;

            let issues = app.query_handler.issue_list().await?;
            println!("{}", serde_json::to_string(&issues)?);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-unblock")]
fn issue_unblock(
    #[opt(name = "issue-id")] issue_id: String,
    #[opt(name = "blocked-issue-id")] blocked_issue_id: String,
    #[opt(long)] command_database_connection_uri: Option<String>,
    #[opt(long)] query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new(
                command_database_connection_uri,
                query_database_connection_uri,
            )
            .await?;
            let use_case = app.issue_management_context_use_case();
            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let blocked_issue_id = IssueId::from_str(blocked_issue_id.as_str())?;
            let issue_block_link_id = IssueBlockLinkId::new(issue_id, blocked_issue_id)?;
            let command = use_case.unblock_issue(issue_block_link_id).into();
            let events = use_case.handle(command).await?;
            // FIXME:
            app.update_query_db(events.first().unwrap().clone()).await?;
            println!("issue unblocked : {:?}", events);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-update")]
fn issue_update(
    issue_id: String,
    #[opt(long = "due")] due: Option<String>,
    #[opt(long)] command_database_connection_uri: Option<String>,
    #[opt(long)] query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new(
                command_database_connection_uri,
                query_database_connection_uri,
            )
            .await?;
            let use_case = app.issue_management_context_use_case();
            let issue_id = IssueId::from_str(issue_id.as_str())?;
            let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
            let command = use_case.update_issue(issue_id, issue_due).into();
            let events = app
                .issue_management_context_use_case()
                .handle(command)
                .await?;
            // FIXME:
            app.update_query_db(events.first().unwrap().clone()).await?;
            println!("issue updated : {:?}", events);
            Ok(())
        })
}

#[argopt::subcmd(name = "issue-view")]
fn issue_view(
    issue_id: String,
    #[opt(long)] command_database_connection_uri: Option<String>,
    #[opt(long)] query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = App::new(
                command_database_connection_uri,
                query_database_connection_uri,
            )
            .await?;
            let issue_id = IssueId::from_str(issue_id.as_str())?;

            let issue = app.query_handler.issue_view(&issue_id).await?;
            println!("{}", serde_json::to_string(&issue)?);
            Ok(())
        })
}

#[derive(Parser)]
struct Opt {
    #[clap(subcommand)]
    resource: Resource,
}

#[derive(Subcommand)]
enum Resource {
    Issue {
        #[clap(subcommand)]
        command: Command,
    },
}

#[derive(Subcommand)]
enum Command {
    Block,
    Create,
    Finish,
    List,
    Unblock,
    Update,
    View,
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
