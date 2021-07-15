use std::process;

use anyhow::{bail, Context};
use chrono::{Local, TimeZone};
use git2::Repository;
use structopt::StructOpt;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Repo {
    name: String,
    path: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Tag {
    date: i64,
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

fn git_tag_list(repository: &Repository) -> anyhow::Result<Vec<String>> {
    let mut tags = vec![];
    let tag_names = repository.tag_names(None)?;
    for i in 0..tag_names.len() {
        let tag_name = tag_names.get(i).context("tag name is not UTF-8")?;
        tags.push(tag_name.to_string());
    }
    Ok(tags)
}

fn time_to_string(timestamp: i64) -> String {
    Local.timestamp(timestamp, 0).to_rfc3339()
}

fn git_log(repository: &Repository, tag_name: &str) -> anyhow::Result<i64> {
    let reference = repository.resolve_reference_from_short_name(tag_name)?;
    let target_oid = reference.target().context("target not found")?;
    let target_object = repository.find_object(target_oid, None)?;
    let commit_object = if let Some(tag) = target_object.as_tag() {
        tag.target()?
    } else {
        target_object
    };
    if let Some(commit) = commit_object.as_commit() {
        Ok(commit.time().seconds())
    } else {
        bail!("not commit {:?} {:?}", commit_object, commit_object.kind())
    }
}

fn list_tags(repo: &Repo) -> anyhow::Result<Vec<Tag>> {
    let repository = Repository::open(&repo.path)?;
    let git_tag_list = git_tag_list(&repository)?;
    let mut tags = vec![];
    for tag_name in git_tag_list {
        let commiter_date = git_log(&repository, tag_name.as_str())?;
        tags.push(Tag {
            date: commiter_date,
            name: tag_name,
        });
    }
    Ok(tags)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "tags", about = "Lists tags")]
struct Opt {
    #[structopt(name = "OWNER")]
    owner: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let repo_list = list_repositories(opt.owner.as_str());
    let mut tags = vec![];
    for repo in repo_list.iter() {
        for tag in list_tags(&repo)? {
            tags.push((repo, tag));
        }
    }
    tags.sort_by_key(|(_, tag)| tag.date);
    for (repo, tag) in tags {
        println!("{} {} {}", time_to_string(tag.date), repo.name, tag.name);
    }
    Ok(())
}
