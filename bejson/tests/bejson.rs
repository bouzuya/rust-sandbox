use assert_cmd::prelude::*;
use std::{fs, process::Command};
use tempfile::tempdir;

#[test]
fn test() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("hello.bejson");
    fs::write(file.as_path(), r#"{"message":$`echo 'Hello,\`bejson\`.'`}"#).unwrap();
    Command::cargo_bin("bejson")
        .unwrap()
        .arg(file.as_path())
        .assert()
        .success()
        .stdout("{\"message\":\"Hello,`bejson`.\\n\"}\n");
}
