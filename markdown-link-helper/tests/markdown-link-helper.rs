use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn main() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    let md = temp_dir.path().join("test.md");
    fs::write(md.as_path(), "[2021-01-02]\n")?;

    let rule = temp_dir.path().join("rule.json");
    fs::write(
        rule.as_path(),
        r#"[["^(\\d{4})-(\\d{2})-(\\d{2})$","[$1-$2-$3]: https://blog.bouzuya.net/$1/$2/$3/"]]"#,
    )?;

    Command::cargo_bin("markdown-link-helper")?
        .arg("--rule-file")
        .arg(rule.as_path())
        .arg(md.as_path())
        .assert()
        .stdout("[2021-01-02]: https://blog.bouzuya.net/2021/01/02/\n");
    Ok(())
}
