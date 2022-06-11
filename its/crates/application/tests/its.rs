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
fn its_issue_block() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "block", "1", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains("issue blocked"))
        .success();
    Ok(())
}

#[test]
fn its_issue_create() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .args(&["issue", "create"])
        .assert()
        .stdout(predicates::str::contains("issue created"))
        .success();
    Ok(())
}

#[test]
fn its_issue_create_with_due() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--due", "2021-02-03T04:05:06+09:00"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains("issue created"))
        .stdout(predicates::str::contains("Instant(1612292706)")) // TODO
        .success();
    Ok(())
}

#[test]
fn its_issue_create_with_title() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains("issue created"))
        .stdout(predicates::str::contains("title1"))
        .success();
    Ok(())
}

#[test]
fn its_issue_finish() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "finish", "1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains("issue finished"))
        .stdout(predicates::str::contains(r#""id":"1""#))
        .success();
    Command::cargo_bin("its")?
        .args(&["issue", "finish", "--resolution", "duplicate", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains("issue finished"))
        .stdout(predicates::str::contains(r#""id":"2""#))
        .stdout(predicates::str::contains(r#""resolution":"duplicate""#))
        .success();
    Ok(())
}

#[test]
fn its_issue_list() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "list"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains(r#""id":"1""#))
        .stdout(predicates::str::contains(r#""id":"2""#))
        .success();
    Ok(())
}

#[test]
fn its_issue_unblock() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "block", "1", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "unblock", "1", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains("issue unblocked"))
        .success();
    Ok(())
}

#[test]
fn its_issue_update() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "update", "--due", "2022-03-04T05:06:07Z", "1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains("issue updated"))
        .stdout(predicates::str::contains("Instant(1646370367)")) // TODO
        .success();
    Ok(())
}

#[test]
fn its_issue_view() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "block", "1", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "view", "1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains(r#""id":"1""#))
        .stdout(predicates::str::contains(r#""title":"title1""#))
        .stdout(predicates::str::contains(
            r#""blocks":[{"id":"2","title":"title2"}]"#,
        ))
        .success();
    Ok(())
}
