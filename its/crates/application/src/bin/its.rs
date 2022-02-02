use std::str::FromStr;

use adapter_fs::FsIssueRepository;
use domain::{IssueDue, IssueId, IssueTitle};
use use_case::{
    CreateIssue, FinishIssue, HasIssueManagementContextUseCase, HasIssueRepository,
    IssueManagementContextCommand, IssueManagementContextUseCase, UpdateIssue,
};

struct App {
    issue_repository: FsIssueRepository,
}

impl App {
    fn new() -> Self {
        Self {
            issue_repository: FsIssueRepository::default(),
        }
    }
}

impl HasIssueRepository for App {
    type IssueRepository = FsIssueRepository;

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
        .build()
        .unwrap()
        .block_on(async {
            let app = App::new();

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
        .build()
        .unwrap()
        .block_on(async {
            let app = App::new();

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

#[argopt::subcmd(name = "issue-update")]
fn issue_update(issue_id: String, #[opt(long = "due")] due: Option<String>) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let app = App::new();

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

#[argopt::cmd_group(commands = [issue_create, issue_finish, issue_update])]
fn main() -> anyhow::Result<()> {}
