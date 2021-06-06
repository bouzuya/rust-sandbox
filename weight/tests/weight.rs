use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn list_and_set_and_remove_test() {
    let dir = tempdir().unwrap();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file")
        .arg(dir.path().join("weight.jsonl").as_path())
        .arg("list")
        .assert()
        .stdout("")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file")
        .arg(dir.path().join("weight.jsonl").as_path())
        .arg("set")
        .arg("2021-02-03")
        .arg("50.1")
        .assert()
        .stdout("")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file")
        .arg(dir.path().join("weight.jsonl").as_path())
        .arg("set")
        .arg("2021-02-04")
        .arg("51.2")
        .assert()
        .stdout("")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file")
        .arg(dir.path().join("weight.jsonl").as_path())
        .arg("list")
        .assert()
        .stdout("2021-02-03 50.1\n2021-02-04 51.2\n")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file")
        .arg(dir.path().join("weight.jsonl").as_path())
        .arg("remove")
        .arg("2021-02-04")
        .assert()
        .stdout("")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file")
        .arg(dir.path().join("weight.jsonl").as_path())
        .arg("list")
        .assert()
        .stdout("2021-02-03 50.1\n")
        .success();
}

#[test]
fn sqlite_test() {
    let dir = tempdir().unwrap();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file-type")
        .arg("sqlite")
        .arg("--data-file")
        .arg(dir.path().join("weight.sqlite").as_path())
        .arg("list")
        .assert()
        .stdout("")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file-type")
        .arg("sqlite")
        .arg("--data-file")
        .arg(dir.path().join("weight.sqlite").as_path())
        .arg("set")
        .arg("2021-02-03")
        .arg("50.1")
        .assert()
        .stdout("")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file-type")
        .arg("sqlite")
        .arg("--data-file")
        .arg(dir.path().join("weight.sqlite").as_path())
        .arg("set")
        .arg("2021-02-04")
        .arg("51.2")
        .assert()
        .stdout("")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file-type")
        .arg("sqlite")
        .arg("--data-file")
        .arg(dir.path().join("weight.sqlite").as_path())
        .arg("list")
        .assert()
        .stdout("2021-02-03 50.1\n2021-02-04 51.2\n")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file-type")
        .arg("sqlite")
        .arg("--data-file")
        .arg(dir.path().join("weight.sqlite").as_path())
        .arg("remove")
        .arg("2021-02-04")
        .assert()
        .stdout("")
        .success();

    Command::cargo_bin("weight")
        .unwrap()
        .arg("--data-file-type")
        .arg("sqlite")
        .arg("--data-file")
        .arg(dir.path().join("weight.sqlite").as_path())
        .arg("list")
        .assert()
        .stdout("2021-02-03 50.1\n")
        .success();
}
#[test]
fn completion_test() {
    Command::cargo_bin("weight")
        .unwrap()
        .arg("completion")
        .arg("fish")
        .assert()
        .stdout(predicates::str::contains("complete -c weight"))
        .success();
}
