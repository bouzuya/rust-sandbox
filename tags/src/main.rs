use std::process;

use anyhow::{bail, Context};
use chrono::{DateTime, Local, TimeZone};
use git2::Repository;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Repo {
    name: String,
    path: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Tag {
    repo: Repo,
    date: String,
    name: String,
}

fn exec_ghq_root() -> String {
    let output = process::Command::new("ghq")
        .arg("root")
        .output()
        .expect("ghq root failed");
    String::from_utf8(output.stdout).expect("ghq root output is not UTF-8")
}

fn exec_ghq_list(owner: &str) -> String {
    let output = process::Command::new("ghq")
        .args(&["list", owner])
        .output()
        .expect("ghq list failed");
    String::from_utf8(output.stdout).expect("ghq list output is not UTF-8")
}

fn exec_git_tag_list(path: &str) -> String {
    let output = process::Command::new("git")
        .args(&["tag", "--list"])
        .current_dir(&path)
        .output()
        .expect("git tag --list failed");
    String::from_utf8(output.stdout).expect("git tag --list is not UTF-8")
}

fn exec_git_log(tag_name: &str, path: &str) -> String {
    let output = process::Command::new("git")
        .args(&["log", "--format=%cI", "--max-count=1", tag_name])
        .current_dir(&path)
        .output()
        .expect("git log failed");
    String::from_utf8(output.stdout).expect("git log output is not UTF-8")
}

fn list_repositories(owner: &str) -> Vec<Repo> {
    let ghq_root = exec_ghq_root();
    let ghq_list = exec_ghq_list(owner);
    ghq_list
        .trim_end()
        .split('\n')
        .map(|item| {
            let mut path = ghq_root.trim_end().to_owned();
            path.push('/');
            path.push_str(item);
            Repo {
                name: path[path.rfind('/').unwrap() + 1..].to_owned(),
                path,
            }
        })
        .collect::<Vec<Repo>>()
}

fn list_tags(repo: &Repo) -> Vec<Tag> {
    let git_tag_list = exec_git_tag_list(&repo.path);
    let mut tags = vec![];
    for tag_name in git_tag_list.trim_end().split('\n') {
        if tag_name.is_empty() {
            continue;
        }
        let date = exec_git_log(tag_name, &repo.path);
        tags.push(Tag {
            repo: repo.clone(),
            date: date.trim_end().to_owned(),
            name: tag_name.to_owned(),
        });
    }
    tags
}

fn git_tag_list(repository: &Repository) -> anyhow::Result<Vec<String>> {
    let mut tags = vec![];
    let tag_names = repository.tag_names(None)?;
    for i in 0..tag_names.len() {
        let tag_name = tag_names.get(i).context("StringArray.get()")?;
        tags.push(tag_name.to_string());
    }
    Ok(tags)
}

fn time_to_string(timestamp: i64) -> String {
    Local.timestamp(timestamp, 0).to_rfc3339()
}

fn git_log(repository: &Repository, tag_name: &str) -> anyhow::Result<String> {
    let reference = repository.resolve_reference_from_short_name(tag_name)?;
    let target_oid = reference.target().context("target not found")?;
    let target_object = repository.find_object(target_oid, None)?;
    let commit_object = if let Some(tag) = target_object.as_tag() {
        tag.target()?
    } else {
        target_object
    };
    let res = if let Some(commit) = commit_object.as_commit() {
        time_to_string(commit.time().seconds())
    } else {
        bail!("not commit {:?} {:?}", commit_object, commit_object.kind())
    };
    Ok(res)
}

fn list_tags3(repo: &Repo) -> anyhow::Result<Vec<Tag>> {
    let repository = Repository::open(&repo.path)?;
    let git_tag_list = git_tag_list(&repository)?;
    let mut tags = vec![];
    for tag_name in git_tag_list {
        let date = git_log(&repository, tag_name.as_str())?;
        if false {
            let date_old = exec_git_log(tag_name.as_str(), &repo.path);
            let d_old = DateTime::parse_from_rfc3339(&date_old.trim_end())?.timestamp();
            let d_new = DateTime::parse_from_rfc3339(&date)?.timestamp();
            assert_eq!(d_new, d_old);
            if d_new != d_old {
                eprintln!("{} {:?}", tag_name, date);
                eprintln!("{} {:?}", tag_name, date_old);
                panic!();
            }
        }
        tags.push(Tag {
            repo: repo.clone(),
            date,
            name: tag_name,
        });
    }
    Ok(tags)
}

fn main() {
    let repo_list = list_repositories("bouzuya");
    let mut tags = vec![];
    for repo in repo_list {
        let mut tag_list = list_tags3(&repo).unwrap(); // TODO
        if false {
            let mut tag_list_old = list_tags(&repo);
            tag_list_old.sort_by_key(|tag| tag.name.clone());
            tag_list.sort_by_key(|tag| tag.name.clone());
            assert_eq!(tag_list, tag_list_old);
            if tag_list != tag_list_old {
                eprintln!("{:?}", tag_list);
                eprintln!("{:?}", tag_list_old);
            }
        }
        tags.append(&mut tag_list);
    }
    tags.sort_by_key(|tag| tag.date.clone());
    for tag in tags {
        println!("{} {} {}", tag.date, tag.repo.name, tag.name);
    }
}
