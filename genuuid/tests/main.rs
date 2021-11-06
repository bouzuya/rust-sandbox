use assert_cmd::Command;
use predicates::str::{contains, is_match};

#[test]
fn subcommand_default_test() -> anyhow::Result<()> {
    Command::cargo_bin("genuuid")?
        .assert()
        .stdout(is_match("^[-0-9a-f]{36}$")?)
        .success();
    Ok(())
}

#[test]
fn subcommand_generate_test() -> anyhow::Result<()> {
    Command::cargo_bin("genuuid")?
        .arg("generate")
        .assert()
        .stdout(is_match("^[-0-9a-f]{36}$")?)
        .success();
    Ok(())
}

#[test]
fn subcommand_help_test() -> anyhow::Result<()> {
    Command::cargo_bin("genuuid")?
        .arg("help")
        .assert()
        .stdout(contains("genuuid"))
        .success();
    Ok(())
}
