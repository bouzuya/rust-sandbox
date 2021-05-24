use assert_cmd::Command;

#[test]
fn test() {
    Command::cargo_bin("bbn")
        .unwrap()
        .arg("list")
        .arg("date:--05-23")
        .assert()
        .success();
}
