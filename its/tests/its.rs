use assert_cmd::Command;

#[test]
fn its_issue_create() -> anyhow::Result<()> {
    Command::cargo_bin("its")?
        .arg("issue-create")
        .assert()
        .stdout(predicates::str::contains("issue created"))
        .success();
    Ok(())
}
