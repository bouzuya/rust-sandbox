use assert_cmd::Command;

#[test]
fn its_no_args() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stderr(predicates::str::contains("USAGE:"))
        .failure();
    Ok(())
}

#[test]
fn its_issue_create() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .arg("issue-create")
        .assert()
        .stdout(predicates::str::contains("issue created"))
        .success();
    Ok(())
}

#[test]
fn its_issue_create_with_title() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue-create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains("issue created"))
        .stdout(predicates::str::contains("title1"))
        .success();
    Ok(())
}
