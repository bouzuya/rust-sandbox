use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use anyhow::bail;
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks, Repository};

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

fn list_repos(users: Vec<PathBuf>) -> anyhow::Result<Vec<PathBuf>> {
    let mut repos = vec![];
    for user in users {
        repos.append(&mut paths(user.as_path())?);
    }
    Ok(repos)
}

pub fn list() -> anyhow::Result<()> {
    let dir = root()?;
    let hosts = list_hosts(dir.as_path())?;
    let users = list_users(hosts)?;
    let mut repos = list_repos(users)?;
    repos.sort();
    for repo in repos {
        println!("{}", repo.strip_prefix(dir.as_path())?.to_string_lossy());
    }
    Ok(())
}
