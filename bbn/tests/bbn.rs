use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn list_test() {
    let temp_dir = tempdir().unwrap();
    let config_dir = temp_dir.path().join("config");
    let data_dir = temp_dir.path().join("data");
    let entry_dir = data_dir.join("2021").join("02");
    fs::create_dir_all(entry_dir.as_path()).unwrap();
    let data20210203 = entry_dir.join("2021-02-03-TITLE.json");
    fs::write(
        data20210203,
        r#"{"minutes":5,"pubdate":"2021-02-03T00:00:00+09:00","tags":[],"title":"TITLE1"}"#,
    )
    .unwrap();
    let hatena_blog_data_file = temp_dir.path().join("hatena-blog.db");

    Command::cargo_bin("bbn")
        .unwrap()
        .arg("config")
        .arg("--data-dir")
        .arg(data_dir)
        .arg("--hatena-blog-data-file")
        .arg(hatena_blog_data_file)
        .env(
            "BBN_TEST_CONFIG_DIR",
            config_dir.as_os_str().to_str().unwrap(),
        )
        .assert()
        .success();
    Command::cargo_bin("bbn")
        .unwrap()
        .arg("list")
        .arg("date:--02-03")
        .env(
            "BBN_TEST_CONFIG_DIR",
            config_dir.as_os_str().to_str().unwrap(),
        )
        .assert()
        .success();
}

#[test]
fn view_test() {
    let temp_dir = tempdir().unwrap();
    let config_dir = temp_dir.path().join("config");
    let data_dir = temp_dir.path().join("data");
    let entry_dir = data_dir.join("2021").join("02");
    fs::create_dir_all(entry_dir.as_path()).unwrap();
    let meta_file = entry_dir.join("2021-02-03-ID_TITLE.json");
    fs::write(
        meta_file,
        r#"{"minutes":5,"pubdate":"2021-02-03T00:00:00+09:00","tags":[],"title":"TITLE1"}"#,
    )
    .unwrap();
    let content_file = entry_dir.join("2021-02-03-ID_TITLE.md");
    fs::write(content_file, r#"Hello"#).unwrap();
    let hatena_blog_data_file = temp_dir.path().join("hatena-blog.db");

    Command::cargo_bin("bbn")
        .unwrap()
        .arg("config")
        .arg("--data-dir")
        .arg(data_dir)
        .arg("--hatena-blog-data-file")
        .arg(hatena_blog_data_file)
        .env(
            "BBN_TEST_CONFIG_DIR",
            config_dir.as_os_str().to_str().unwrap(),
        )
        .assert()
        .success();
    Command::cargo_bin("bbn")
        .unwrap()
        .arg("view")
        .arg("2021-02-03")
        .env(
            "BBN_TEST_CONFIG_DIR",
            config_dir.as_os_str().to_str().unwrap(),
        )
        .assert()
        .success()
        .stdout("2021-02-03 ID_TITLE TITLE1 https://blog.bouzuya.net/2021/02/03/\nHello\n");
}
