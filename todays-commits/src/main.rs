use std::ops::RangeInclusive;

use anyhow::Result;
use chrono::{Date, DateTime, Local, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RepoResponse {
    name: String,
    html_url: String,
    pushed_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CommitResponse {
    sha: String,
    html_url: String,
    commit: Commit,
}

#[derive(Debug, Deserialize)]
struct Commit {
    message: String,
    committer: Committer,
}

#[derive(Debug, Deserialize)]
struct Committer {
    date: DateTime<Utc>,
}

fn main() -> Result<()> {
    let owner = "bouzuya";
    let sort = GetReposSort::Pushed;
    let repos = get_repos(owner, &sort)?;
    let today = Local::today().format("%Y-%m-%d").to_string();
    let range = build_range(&today)?;
    let mut repo_li = vec![];
    for repo in repos.iter().filter(|repo| {
        range.contains(&match sort {
            GetReposSort::Pushed => repo.pushed_at,
            GetReposSort::Updated => repo.updated_at,
        })
    }) {
        let mut commit_li = vec![];
        let commits = get_commits(owner, &repo.name)?;
        for commit in commits
            .iter()
            .filter(|commit| range.contains(&commit.commit.committer.date))
        {
            commit_li.push(format!(
                "  - [{}]({})",
                commit.commit.message.split('\n').nth(0).unwrap(),
                commit.html_url
            ));
        }
        repo_li.push((
            format!(
                "- [{}]({}) {} commit{}",
                repo.name,
                repo.html_url,
                commit_li.len(),
                if commit_li.len() > 1 { "s" } else { "" }
            ),
            commit_li,
        ));
    }

    print_formatted(&repo_li);

    Ok(())
}

fn print_formatted(formatted: &Vec<(String, Vec<String>)>) {
    for repo in formatted {
        println!("{}", repo.0);
        for commit in repo.1.iter() {
            println!("{}", commit);
        }
    }
}

fn get_commits(owner: &str, repo: &str) -> Result<Vec<CommitResponse>> {
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

#[derive(Debug, Eq, PartialEq)]
enum GetReposSort {
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

fn get_repos(owner: &str, sort: &GetReposSort) -> Result<Vec<RepoResponse>> {
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

fn build_range(s: &str) -> Result<RangeInclusive<DateTime<Utc>>> {
    let today = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
    let offset = Local::now().offset().clone();
    let today = Date::<Local>::from_utc(today, offset);
    let today_start = today.and_hms(0, 0, 0);
    let today_end = today.and_hms(23, 59, 59);
    let start = today_start.with_timezone(&Utc);
    let end = today_end.with_timezone(&Utc);
    let range = start..=end;
    Ok(range)
}
