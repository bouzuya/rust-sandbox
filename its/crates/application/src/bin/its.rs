use std::{
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use adapter_sqlite::{
    RdbConnectionPool, SqliteIssueBlockLinkRepository, SqliteIssueCommentRepository,
    SqliteIssueRepository,
};
use adapter_sqlite_query::SqliteQueryHandler;
use anyhow::Context;
use clap::{Parser, Subcommand};
use domain::{
    aggregate::issue::{IssueDescription, IssueDue, IssueResolution, IssueTitle},
    IssueBlockLinkId, IssueId,
};
use use_case::{
    issue_comment_repository::HasIssueCommentRepository, BlockIssue, CreateIssue, FinishIssue,
    HasIssueBlockLinkRepository, HasIssueManagementContextUseCase, HasIssueRepository,
    IssueManagementContextUseCase, UnblockIssue, UpdateIssue, UpdateIssueDescription,
    UpdateIssueTitle,
};
use xdg::BaseDirectories;

struct App {
    issue_block_link_repository: SqliteIssueBlockLinkRepository,
    issue_comment_repository: SqliteIssueCommentRepository,
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
        let issue_comment_repository = connection_pool.issue_comment_repository()?;
        let issue_repository = connection_pool.issue_repository()?;
        let issue_block_link_repository = connection_pool.issue_block_link_repository()?;
        let query_handler = SqliteQueryHandler::new(
            &query_connection_uri,
            connection_pool.clone(),
            Arc::new(Mutex::new(issue_repository)),
            Arc::new(Mutex::new(issue_block_link_repository)),
        )
        .await?;
        let issue_repository = connection_pool.issue_repository()?;
        let issue_block_link_repository = connection_pool.issue_block_link_repository()?;
        Ok(Self {
            issue_block_link_repository,
            issue_comment_repository,
            issue_repository,
            query_handler,
        })
    }

    // TODO: remove
    async fn update_query_db(&self) -> anyhow::Result<()> {
        self.query_handler.update_database().await?;
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

impl HasIssueCommentRepository for App {
    type IssueCommentRepository = SqliteIssueCommentRepository;

    fn issue_comment_repository(&self) -> &Self::IssueCommentRepository {
        &self.issue_comment_repository
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

async fn issue_block(
    issue_id: String,
    blocked_issue_id: String,
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;
    let use_case = app.issue_management_context_use_case();
    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let blocked_issue_id = IssueId::from_str(blocked_issue_id.as_str())?;
    let event = use_case
        .handle(BlockIssue {
            issue_id,
            blocked_issue_id,
        })
        .await?;
    if let use_case::IssueManagementContextEvent::IssueBlocked {
        issue_block_link_id,
    } = event
    {
        // FIXME:
        app.update_query_db().await?;
        let issue_id = issue_block_link_id.issue_id();
        let issue = app.query_handler.issue_view(issue_id).await?.unwrap();
        println!("{}", serde_json::to_string(&issue)?);
    }
    Ok(())
}

async fn issue_create(
    title: Option<String>,
    due: Option<String>,
    description: Option<String>,
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;
    let use_case = app.issue_management_context_use_case();
    let issue_title = IssueTitle::try_from(title.unwrap_or_default())?;
    let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
    let issue_description = description
        .map(|s| IssueDescription::from_str(s.as_str()))
        .transpose()?
        .unwrap_or_default();
    let event = use_case
        .handle(CreateIssue {
            issue_title,
            issue_due,
            issue_description,
        })
        .await?;
    if let use_case::IssueManagementContextEvent::IssueCreated { issue_id } = event {
        // FIXME:
        app.update_query_db().await?;
        let issue = app.query_handler.issue_view(&issue_id).await?.unwrap();
        println!("{}", serde_json::to_string(&issue)?);
    }
    Ok(())
}

async fn issue_update_description(
    issue_id: String,
    description: String,
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;
    let use_case = app.issue_management_context_use_case();
    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let issue_description = IssueDescription::try_from(description)?;
    use_case
        .handle(UpdateIssueDescription {
            issue_id: issue_id.clone(),
            issue_description,
        })
        .await?;
    // FIXME:
    app.update_query_db().await?;
    let issue = app.query_handler.issue_view(&issue_id).await?.unwrap();
    println!("{}", serde_json::to_string(&issue)?);
    Ok(())
}

async fn issue_update_title(
    issue_id: String,
    title: String,
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;
    let use_case = app.issue_management_context_use_case();
    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let issue_title = IssueTitle::try_from(title)?;
    use_case
        .handle(UpdateIssueTitle {
            issue_id: issue_id.clone(),
            issue_title,
        })
        .await?;
    // FIXME:
    app.update_query_db().await?;
    let issue = app.query_handler.issue_view(&issue_id).await?.unwrap();
    println!("{}", serde_json::to_string(&issue)?);
    Ok(())
}

async fn issue_finish(
    issue_id: String,
    resolution: Option<String>,
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;
    let use_case = app.issue_management_context_use_case();
    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let resolution = resolution
        .as_deref()
        .map(IssueResolution::from_str)
        .transpose()?;
    use_case
        .handle(FinishIssue {
            issue_id: issue_id.clone(),
            resolution,
        })
        .await?;
    // FIXME:
    app.update_query_db().await?;
    let issue = app.query_handler.issue_view(&issue_id).await?.unwrap();
    println!("{}", serde_json::to_string(&issue)?);
    Ok(())
}

async fn issue_list(
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;

    let issues = app.query_handler.issue_list().await?;
    println!("{}", serde_json::to_string(&issues)?);
    Ok(())
}

async fn issue_unblock(
    issue_id: String,
    blocked_issue_id: String,
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;
    let use_case = app.issue_management_context_use_case();
    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let blocked_issue_id = IssueId::from_str(blocked_issue_id.as_str())?;
    let issue_block_link_id = IssueBlockLinkId::new(issue_id, blocked_issue_id)?;
    let event = use_case
        .handle(UnblockIssue {
            issue_block_link_id,
        })
        .await?;
    if let use_case::IssueManagementContextEvent::IssueUnblocked {
        issue_block_link_id,
    } = event
    {
        // FIXME:
        app.update_query_db().await?;
        let issue_id = issue_block_link_id.issue_id();
        let issue = app.query_handler.issue_view(issue_id).await?.unwrap();
        println!("{}", serde_json::to_string(&issue)?);
    }
    Ok(())
}

async fn issue_update(
    issue_id: String,
    due: Option<String>,
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;
    let use_case = app.issue_management_context_use_case();
    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
    use_case
        .handle(UpdateIssue {
            issue_id: issue_id.clone(),
            issue_due,
        })
        .await?;
    // FIXME:
    app.update_query_db().await?;
    let issue = app.query_handler.issue_view(&issue_id).await?.unwrap();
    println!("{}", serde_json::to_string(&issue)?);
    Ok(())
}

async fn issue_view(
    issue_id: String,
    command_database_connection_uri: Option<String>,
    query_database_connection_uri: Option<String>,
) -> anyhow::Result<()> {
    let app = App::new(
        command_database_connection_uri,
        query_database_connection_uri,
    )
    .await?;
    let issue_id = IssueId::from_str(issue_id.as_str())?;

    let issue = app.query_handler.issue_view(&issue_id).await?;
    println!("{}", serde_json::to_string(&issue)?);
    Ok(())
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
    Block {
        #[clap(name = "issue-id")]
        issue_id: String,
        #[clap(name = "blocked-issue-id")]
        blocked_issue_id: String,
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
    Create {
        #[clap(long = "title")]
        title: Option<String>,
        #[clap(long = "due")]
        due: Option<String>,
        #[clap(long = "description")]
        description: Option<String>,
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
    Finish {
        issue_id: String,
        #[clap(long)]
        resolution: Option<String>,
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
    List {
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
    Unblock {
        #[clap(name = "issue-id")]
        issue_id: String,
        #[clap(name = "blocked-issue-id")]
        blocked_issue_id: String,
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
    Update {
        issue_id: String,
        #[clap(long = "due")]
        due: Option<String>,
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
    UpdateDescription {
        issue_id: String,
        description: String,
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
    UpdateTitle {
        issue_id: String,
        title: String,
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
    View {
        issue_id: String,
        #[clap(long)]
        command_database_connection_uri: Option<String>,
        #[clap(long)]
        query_database_connection_uri: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    match opt.resource {
        Resource::Issue { command } => match command {
            Command::Block {
                issue_id,
                blocked_issue_id,
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_block(
                    issue_id,
                    blocked_issue_id,
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
            Command::Create {
                title,
                due,
                description,
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_create(
                    title,
                    due,
                    description,
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
            Command::Finish {
                issue_id,
                resolution,
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_finish(
                    issue_id,
                    resolution,
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
            Command::List {
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_list(
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
            Command::Unblock {
                issue_id,
                blocked_issue_id,
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_unblock(
                    issue_id,
                    blocked_issue_id,
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
            Command::Update {
                issue_id,
                due,
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_update(
                    issue_id,
                    due,
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
            Command::UpdateDescription {
                issue_id,
                description,
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_update_description(
                    issue_id,
                    description,
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
            Command::UpdateTitle {
                issue_id,
                title,
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_update_title(
                    issue_id,
                    title,
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
            Command::View {
                issue_id,
                command_database_connection_uri,
                query_database_connection_uri,
            } => {
                issue_view(
                    issue_id,
                    command_database_connection_uri,
                    query_database_connection_uri,
                )
                .await
            }
        },
    }
}
