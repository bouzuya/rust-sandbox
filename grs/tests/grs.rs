use std::fs;

use assert_cmd::Command;
use predicates::str::starts_with;
use tempfile::tempdir;

#[test]
fn list_test() -> anyhow::Result<()> {
    let temp_dir = tempdir().unwrap();
    let root_dir = temp_dir.path().join("grs");
    fs::create_dir_all(root_dir.as_path())?;
    Command::cargo_bin("grs")
        .unwrap()
        .arg("list")
        .env_clear()
        .env("GRS_ROOT", root_dir.as_os_str())
        .assert()
        .stdout("")
        .success();
    Command::cargo_bin("grs")
        .unwrap()
        .arg("list")
        .env_clear()
        .env("HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout("")
        .success();
    Command::cargo_bin("grs")
        .unwrap()
        .arg("list")
        .env_clear()
        .assert()
        .stderr(starts_with("Error: environment variable not found\n"))
        .failure();
    Ok(())
}

#[test]
fn list_query_test() -> anyhow::Result<()> {
    // TODO
    Ok(())
}

#[test]
fn list_full_path_test() -> anyhow::Result<()> {
    // TODO
    Ok(())
}

#[test]
fn root_test() -> anyhow::Result<()> {
    let temp_dir = tempdir().unwrap();
    let root_dir = temp_dir.path().join("grs");
    fs::create_dir_all(root_dir.as_path())?;
    Command::cargo_bin("grs")
        .unwrap()
        .arg("root")
        .env_clear()
        .env("GRS_ROOT", root_dir.as_os_str())
        .assert()
        .stdout(format!("{}\n", root_dir.display()))
        .success();
    Command::cargo_bin("grs")
        .unwrap()
        .arg("root")
        .env_clear()
        .env("HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(format!("{}\n", root_dir.display()))
        .success();
    Command::cargo_bin("grs")
        .unwrap()
        .arg("root")
        .env_clear()
        .assert()
        .stderr(starts_with("Error: environment variable not found\n"))
        .failure();
    Ok(())
}
