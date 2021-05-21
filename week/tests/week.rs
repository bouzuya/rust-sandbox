use assert_cmd::Command;

#[test]
fn no_options() {
    Command::cargo_bin("week")
        .unwrap()
        .arg("2021-05-22")
        .assert()
        .stdout("2021-W20\n")
        .success();
}

#[test]
fn week_date_option() {
    Command::cargo_bin("week")
        .unwrap()
        .arg("--week-date")
        .arg("2021-05-22")
        .assert()
        .stdout("2021-W20-6\n")
        .success();
}

#[test]
fn week_option() {
    Command::cargo_bin("week")
        .unwrap()
        .arg("--week")
        .arg("2021-05-22")
        .assert()
        .stdout("2021-W20\n")
        .success();
}

#[test]
fn week_year_option() {
    Command::cargo_bin("week")
        .unwrap()
        .arg("--week-year")
        .arg("2021-05-22")
        .assert()
        .stdout("2021\n")
        .success();
}
