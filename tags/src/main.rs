use std::process;

#[derive(Clone, Debug)]
struct Repo {
    name: String,
    path: String,
}

#[derive(Debug)]
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
            path.push_str("/");
            path.push_str(item);
            Repo {
                name: path[path.rfind("/").unwrap() + 1..].to_owned(),
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

fn main() {
    let repo_list = list_repositories("bouzuya");
    let mut tags = vec![];
    for repo in repo_list {
        let mut tag_list = list_tags(&repo);
        tags.append(&mut tag_list);
    }
    tags.sort_by_key(|tag| tag.date.clone());
    for tag in tags {
        println!("{} {} {}", tag.date, tag.repo.name, tag.name);
    }
}
