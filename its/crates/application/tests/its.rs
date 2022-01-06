use assert_cmd::Command;

#[test]
fn its_no_args() -> anyhow::Result<()> {
    Command::cargo_bin("its")?
        .assert()
        .stderr(predicates::str::contains("USAGE:"))
        .failure();
    Ok(())
}

#[test]
fn its_issue_create() -> anyhow::Result<()> {
    Command::cargo_bin("its")?
        .arg("issue-create")
        .assert()
        .stdout(predicates::str::contains("issue created"))
        .success();
    Ok(())
}

#[test]
fn its_issue_create_with_title() -> anyhow::Result<()> {
    Command::cargo_bin("its")?
        .args(&["issue-create", "--title", "title1"])
        .assert()
        .stdout(predicates::str::contains("issue created"))
        .stdout(predicates::str::contains("title1"))
        .success();
    Ok(())
}
