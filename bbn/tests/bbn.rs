use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn list_test() {
    let dir = tempdir().unwrap();
    let data = dir.path().join("data");
    let data2021 = data.join("2021");
    let data202102 = data2021.join("02");
    fs::create_dir_all(data202102.as_path()).unwrap();
    let data20210203 = data202102.join("2021-02-03-TITLE.json");
    fs::write(data20210203, r#"{"title":"TITLE1"}"#).unwrap();

    Command::cargo_bin("bbn")
        .unwrap()
        .arg("list")
        .arg("--data-dir")
        .arg(data.canonicalize().unwrap().as_path().to_str().unwrap())
        .arg("date:--02-03")
        .assert()
        .success();
}

#[test]
fn view_test() {
    let dir = tempdir().unwrap();
    let data = dir.path().join("data");
    let data2021 = data.join("2021");
    let data202102 = data2021.join("02");
    fs::create_dir_all(data202102.as_path()).unwrap();
    let data20210203 = data202102.join("2021-02-03-TITLE.json");
    fs::write(data20210203, r#"{"title":"TITLE1"}"#).unwrap();

    Command::cargo_bin("bbn")
        .unwrap()
        .arg("view")
        .arg("--data-dir")
        .arg(data.canonicalize().unwrap().as_path().to_str().unwrap())
        .arg("2021-02-03")
        .assert()
        .success()
        .stdout("2021-02-03 TITLE1 https://blog.bouzuya.net/2021/02/03/\n");
}
