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
        .assert()
        .stdout(predicates::str::contains(r#""blocks":[]"#));
    Command::cargo_bin("its")?
        .args(&["issue", "block", "1", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains(r#""blocks":[{"id":"2""#))
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
        .stdout(predicates::str::contains(r#""id":"#))
        .success();
    Ok(())
}

#[test]
fn its_issue_create_with_description() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--description", "desc1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains(r#""description":"desc1""#))
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
        .stdout(predicates::str::contains("2021-02-02T19:05:06Z"))
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
        .stdout(predicates::str::contains(r#""id":"1""#))
        .stdout(predicates::str::contains(r#""resolution":null"#))
        .stdout(predicates::str::contains(r#""status":"done""#))
        .success();
    Command::cargo_bin("its")?
        .args(&["issue", "finish", "--resolution", "duplicate", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains(r#""id":"2""#))
        .stdout(predicates::str::contains(r#""resolution":"duplicate""#))
        .stdout(predicates::str::contains(r#""status":"done""#))
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
        .assert()
        .stdout(predicates::str::contains(r#""blocks":[]"#));
    Command::cargo_bin("its")?
        .args(&["issue", "block", "1", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains(r#""blocks":[{"id":"2""#));
    Command::cargo_bin("its")?
        .args(&["issue", "unblock", "1", "2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        // FIXME
        // .stdout(predicates::str::contains(r#""blocks":[]"#))
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
        .stdout(predicates::str::contains("2022-03-04T05:06:07Z"))
        .success();
    Ok(())
}

#[test]
fn its_issue_update_description() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "update-description", "1", "desc2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains(r#""description":"desc2""#))
        .success();
    Ok(())
}

#[test]
fn its_issue_update_title() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    Command::cargo_bin("its")?
        .args(&["issue", "create", "--title", "title1"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert();
    Command::cargo_bin("its")?
        .args(&["issue", "update-title", "1", "title2"])
        .env("XDG_STATE_HOME", temp_dir.path().as_os_str())
        .assert()
        .stdout(predicates::str::contains(r#""title":"title2""#))
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
