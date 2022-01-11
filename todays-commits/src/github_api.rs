use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommitResponse {
    pub sha: String,
    pub html_url: String,
    pub commit: Commit,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub message: String,
    pub committer: Committer,
}

#[derive(Debug, Deserialize)]
pub struct Committer {
    pub date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RepoResponse {
    pub name: String,
    pub html_url: String,
    pub pushed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum GetReposSort {
    Pushed,
    Updated,
}

impl std::fmt::Display for GetReposSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GetReposSort::Pushed => "pushed",
                GetReposSort::Updated => "updated",
            }
        )
    }
}

pub fn get_commits(owner: &str, repo: &str) -> anyhow::Result<Vec<CommitResponse>> {
    let url = format!("https://api.github.com/repos/{}/{}/commits", owner, repo);
    let client = reqwest::blocking::Client::new();
    let request = client
        .get(&url)
        .header("User-Agent", "todays-commits")
        .build()?;
    let response = client.execute(request)?;
    let repos = response.json()?;
    Ok(repos)
}

pub fn get_repos(owner: &str, sort: &GetReposSort) -> anyhow::Result<Vec<RepoResponse>> {
    let url = format!("https://api.github.com/users/{}/repos?sort={}", owner, sort);
    let client = reqwest::blocking::Client::new();
    let request = client
        .get(&url)
        .header("User-Agent", "todays-commits")
        .build()?;
    let response = client.execute(request)?;
    let repos = response.json()?;
    Ok(repos)
}
