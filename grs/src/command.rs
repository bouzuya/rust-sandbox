mod root;

use self::root::root;

use anyhow::bail;
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "get", about = "Clone a repository")]
    Get {
        #[structopt(name = "USER/REPO")]
        name: String,
    },
    #[structopt(name = "root", about = "Prints root")]
    Root,
}

pub fn run() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Get { name } => {
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
        }
        Subcommand::Root => {
            println!("{:?}", root()?);
        }
    }
    Ok(())
}
