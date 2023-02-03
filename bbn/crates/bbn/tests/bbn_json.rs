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
    fs::write(
        content20210203,
        concat!(
            r#"hello [2021-02-04]"#,
            "\n",
            r#"[2021-02-04]: https://blog.bouzuya.net/2021/02/04/"#
        ),
    )?;
    let meta20210203 = entry_dir.join("2021-02-04.json");
    fs::write(
        meta20210203,
        r#"{"minutes":5,"pubdate":"2021-02-04T00:00:00+09:00","tags":["tag1"],"title":"TITLE2"}"#,
    )?;
    let content20210203 = entry_dir.join("2021-02-04.md");
    fs::write(content20210203, r#"good bye"#)?;

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
        r#"[{"date":"2021-02-03","minutes":5,"pubdate":"2021-02-03T00:00:00+09:00","tags":[],"title":"TITLE1"},{"date":"2021-02-04","minutes":5,"pubdate":"2021-02-04T00:00:00+09:00","tags":["tag1"],"title":"TITLE2"}]"#
    );

    let daily_json_content = r#"{"data":"hello [2021-02-04]\n[2021-02-04]: https://blog.bouzuya.net/2021/02/04/","date":"2021-02-03","minutes":5,"html":"<p>hello [2021-02-04]\n[2021-02-04]: https://blog.bouzuya.net/2021/02/04/</p>\n","pubdate":"2021-02-03T00:00:00+09:00","tags":[],"title":"TITLE1"}"#;
    assert_eq!(
        fs::read_to_string(out_dir.join("2021/02/03.json"))?,
        daily_json_content
    );
    assert_eq!(
        fs::read_to_string(out_dir.join("2021/02/03/index.json"))?,
        daily_json_content
    );
    assert_eq!(
        fs::read_to_string(out_dir.join("2021/02/03/TITLE.json"))?,
        daily_json_content
    );
    assert_eq!(
        fs::read_to_string(out_dir.join("2021/02/03/TITLE/index.json"))?,
        daily_json_content
    );

    assert_eq!(
        fs::read_to_string(out_dir.join("linked.json"))?,
        r#"{"2021-02-04":["2021-02-03"]}"#
    );
    assert_eq!(
        fs::read_to_string(out_dir.join("tags.json"))?,
        r#"[{"name":"tag1","count":1}]"#
    );

    Ok(())
}
