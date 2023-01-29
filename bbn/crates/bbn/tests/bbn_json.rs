use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_bbn_json() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    let config_dir = temp_dir.path().join("config");
    let data_dir = temp_dir.path().join("data");
    let entry_dir = data_dir.join("2021").join("02");
    fs::create_dir_all(entry_dir.as_path())?;
    let meta20210203 = entry_dir.join("2021-02-03-TITLE.json");
    fs::write(
        meta20210203,
        r#"{"minutes":5,"pubdate":"2021-02-03T00:00:00+09:00","tags":[],"title":"TITLE1"}"#,
    )?;
    let content20210203 = entry_dir.join("2021-02-03-TITLE.md");
    fs::write(content20210203, r#"hello"#)?;

    let hatena_blog_data_file = temp_dir.path().join("hatena-blog.db");
    Command::cargo_bin("bbn")?
        .arg("config")
        .arg("--data-dir")
        .arg(data_dir)
        .arg("--hatena-blog-data-file")
        .arg(hatena_blog_data_file)
        .env("BBN_TEST_CONFIG_DIR", config_dir.as_path())
        .assert()
        .success();

    let out_dir = temp_dir.path().join("out");
    fs::create_dir_all(out_dir.as_path())?;

    Command::cargo_bin("bbn")?
        .arg("json")
        .arg(out_dir.as_path())
        .env("BBN_TEST_CONFIG_DIR", config_dir)
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(out_dir.join("posts.json"))?,
        r#"[{"date":"2021-02-03","minutes":5,"pubdate":"2021-02-03T00:00:00+09:00","tags":[],"title":"TITLE1"}]"#
    );

    // TODO: daily json
    // TODO: linked.json
    // TODO: tags.json
    Ok(())
}
