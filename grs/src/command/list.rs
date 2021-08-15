use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::command::root;

fn paths(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut paths = vec![];
    for dir_entry in read_dir(dir)? {
        let dir_entry = dir_entry?;
        if dir_entry.file_type()?.is_dir() {
            paths.push(dir.join(dir_entry.path()));
        }
    }
    Ok(paths)
}

fn list_hosts(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    paths(dir)
}

fn list_users(hosts: Vec<PathBuf>) -> anyhow::Result<Vec<PathBuf>> {
    let mut users = vec![];
    for host in hosts {
        users.append(&mut paths(host.as_path())?);
    }
    Ok(users)
}

fn list_repos(users: Vec<PathBuf>, query: Option<String>) -> anyhow::Result<Vec<PathBuf>> {
    let mut repos = vec![];
    for user in users {
        for repo in paths(user.as_path())? {
            if repo.join(".git").is_dir() {
                match query {
                    Some(ref q) => match user.file_name().zip(repo.file_name()) {
                        None => continue,
                        Some((u, r)) => {
                            let s = format!("{}/{}", u.to_string_lossy(), r.to_string_lossy());
                            if s.contains(q) {
                                repos.push(repo)
                            }
                        }
                    },
                    None => {
                        repos.push(repo);
                    }
                }
            }
        }
    }
    Ok(repos)
}

pub fn list(query: Option<String>) -> anyhow::Result<()> {
    let dir = root()?;
    let hosts = list_hosts(dir.as_path())?;
    let users = list_users(hosts)?;
    let mut repos = list_repos(users, query)?;
    repos.sort();
    for repo in repos {
        println!("{}", repo.strip_prefix(dir.as_path())?.to_string_lossy());
    }
    Ok(())
}
