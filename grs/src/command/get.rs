use anyhow::bail;
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};

use crate::command::root;

pub fn get(name: String) -> anyhow::Result<()> {
    let user_project = name.split('/').collect::<Vec<&str>>();
    if user_project.len() != 2 {
        bail!("USER/REPO");
    }
    let user = user_project[0];
    let project = user_project[1];
    let dir = root()?.join("github.com").join(user);
    let url = format!("git@github.com:{}/{}.git", user, project);
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap())
    });
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);
    builder.clone(url.as_str(), dir.as_path())?;
    Ok(())
}
