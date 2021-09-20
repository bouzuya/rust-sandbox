use assert_cmd::Command;
use predicates::str::contains;
use std::fs;

#[test]
fn completion_test() {
    Command::cargo_bin("b")
        .unwrap()
        .arg("completion")
        .arg("fish")
        .assert()
        .stdout(contains("complete"))
        .success();
}

#[test]
fn new_test() {
    let dir = tempfile::tempdir().unwrap();
    let tmpl_dir = dir.path().join("tmpl");
    fs::create_dir(tmpl_dir.as_path()).unwrap();
    fs::write(tmpl_dir.join("{{foo}}.md").as_path(), "{{bar}}").unwrap();

    Command::cargo_bin("b")
        .unwrap()
        .arg("new")
        .arg("--template")
        .arg("./tmpl")
        .arg("--data-file")
        .arg("-")
        .write_stdin(r#"{"foo":"FOO","bar":"BAR"}"#)
        .current_dir(dir.path())
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(dir.path().join("FOO.md").as_path()).unwrap(),
        "BAR".to_string()
    );
}

#[test]
fn list_test() {
    let dir = tempfile::tempdir().unwrap();
    let b_dir = dir.path().join("flow").join("2021").join("02").join("03");
    fs::create_dir_all(b_dir.as_path()).unwrap();
    let md = b_dir.join("20210203T000000Z.md");
    fs::write(md.as_path(), "markdown").unwrap();
    let json = b_dir.join("20210203T000000Z.json");
    fs::write(
        json.as_path(),
        r#"{"created_at":"2021-02-03T09:00:00+09:00"}"#,
    )
    .unwrap();

    Command::cargo_bin("b")
        .unwrap()
        .arg("list")
        .arg("--data-dir")
        .arg(dir.path())
        .arg("2021-02-03")
        .assert()
        .stdout(contains(md.as_path().to_str().unwrap()))
        .success();
}
