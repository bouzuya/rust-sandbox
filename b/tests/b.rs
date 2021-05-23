use assert_cmd::Command;
use std::fs;

#[test]
fn test() {
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
