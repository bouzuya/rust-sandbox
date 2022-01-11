mod github_api;

use std::ops::RangeInclusive;

use anyhow::Result;
use chrono::{Date, DateTime, Local, NaiveDate, Utc};

use crate::github_api::{get_commits, get_repos, GetReposSort};

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
                commit.commit.message.split('\n').next().unwrap(),
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

fn print_formatted(formatted: &[(String, Vec<String>)]) {
    for repo in formatted {
        println!("{}", repo.0);
        for commit in repo.1.iter() {
            println!("{}", commit);
        }
    }
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
