mod github_api;

use std::ops::RangeInclusive;

use anyhow::Result;
use argopt::cmd;
use chrono::{Date, DateTime, Local, NaiveDate, Utc};
use github_api::RepoResponse;

use crate::github_api::{get_commits, get_repos, CommitResponse, GetReposSort};

type RepoAndCommits = (RepoResponse, Vec<CommitResponse>);

fn fetch(
    owner: &str,
    sort: GetReposSort,
    range: RangeInclusive<DateTime<Utc>>,
) -> Result<Vec<RepoAndCommits>> {
    let repos = get_repos(owner, &sort)?;
    let mut repo_and_commits = vec![];
    for repo in repos.into_iter().filter(|repo| {
        range.contains(&match sort {
            GetReposSort::Pushed => repo.pushed_at,
            GetReposSort::Updated => repo.updated_at,
        })
    }) {
        let commits = get_commits(owner, &repo.name)?;
        let filtered = commits
            .into_iter()
            .filter(|commit| range.contains(&commit.commit.committer.date))
            .collect::<Vec<CommitResponse>>();
        repo_and_commits.push((repo, filtered));
    }
    Ok(repo_and_commits)
}

fn format(repo_and_commits: Vec<RepoAndCommits>) -> String {
    let mut formatted = String::new();
    for (repo, commits) in repo_and_commits {
        let mut commit_li = vec![];
        for commit in commits {
            commit_li.push(format!(
                "  - [{}]({})",
                commit.commit.message.split('\n').next().unwrap(),
                commit.html_url
            ));
        }
        formatted.push_str(&format!(
            "- [{}]({}) {} commit{}\n{}\n",
            repo.name,
            repo.html_url,
            commit_li.len(),
            if commit_li.len() > 1 { "s" } else { "" },
            commit_li.join("\n"),
        ));
    }
    formatted
}

#[derive(Debug, Eq, PartialEq)]
pub enum Format {
    Json,
    Text,
}

impl std::str::FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "text" => Ok(Self::Text),
            _ => Err(anyhow::anyhow!("unknown format")),
        }
    }
}

#[cmd]
fn main(
    #[opt(long = "sort", default_value = "pushed")] sort: GetReposSort,
    #[opt(long = "format", default_value = "text")] output_format: Format,
) -> Result<()> {
    let today = Local::today().format("%Y-%m-%d").to_string();
    let range = build_range(&today)?;
    let repo_and_commits = fetch("bouzuya", sort, range)?;
    let formatted = format(repo_and_commits);
    let output = match output_format {
        Format::Json => serde_json::to_string(&formatted)?,
        Format::Text => formatted,
    };
    println!("{}", output);
    Ok(())
}

fn build_range(s: &str) -> Result<RangeInclusive<DateTime<Utc>>> {
    let today = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
    let offset = *Local::now().offset();
    let today = Date::<Local>::from_utc(today, offset);
    let today_start = today.and_hms(0, 0, 0);
    let today_end = today.and_hms(23, 59, 59);
    let start = today_start.with_timezone(&Utc);
    let end = today_end.with_timezone(&Utc);
    let range = start..=end;
    Ok(range)
}
