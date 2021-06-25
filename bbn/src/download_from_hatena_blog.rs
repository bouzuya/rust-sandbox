use std::{collections::BTreeSet, fs, path::PathBuf, time::Duration};

use hatena_blog::{Client, Config};
use tokio::time::sleep;

pub async fn download_from_hatena_blog(
    data_file: PathBuf,
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
) -> anyhow::Result<()> {
    let config = Config::new(
        &hatena_id,
        "https://blog.hatena.ne.jp",
        &hatena_blog_id,
        &hatena_api_key,
    );
    let client = Client::new(&config);
    let mut set = BTreeSet::new();
    let mut next_page = None;
    loop {
        match client.list_entries_in_page(next_page.as_deref()).await? {
            (None, entry_ids) => {
                for entry_id in entry_ids {
                    set.insert(entry_id.to_string());
                }
                break;
            }
            (Some(page), entry_ids) => {
                for entry_id in entry_ids {
                    set.insert(entry_id.to_string());
                }
                next_page = Some(page);
            }
        }
        let _ = fs::write(
            data_file.as_path(),
            set.iter().cloned().collect::<Vec<String>>().join("\n"),
        );
        eprintln!("{}", set.len());
        sleep(Duration::from_secs(1)).await;
    }

    for entry_id in set {
        println!("{}", entry_id);
    }

    Ok(())
}
