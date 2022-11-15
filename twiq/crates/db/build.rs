use std::{fs, path::Path};

use prost_build::Config;

fn main() -> anyhow::Result<()> {
    // brew install protobuf
    let out_dir = Path::new("src/firestore_rpc");
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)?;
    }
    fs::create_dir_all(&out_dir)?;
    // コメントに非 Rust の fenced code block があることで doctest が壊れるので
    // 一部のコメントの出力を避ける
    let mut config = Config::new();
    // message および field のコメント出力の無効化
    config.disable_comments(&[
        "google.api.HttpRule",
        "google.firestore.v1.StructuredQuery.start_at",
        "google.firestore.v1.StructuredAggregationQuery.Aggregation.alias",
        "google.firestore.v1.StructuredAggregationQuery.Aggregation.Count.up_to",
    ]);
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        // service および rpc のコメント出力の無効化
        .disable_comments("google.firestore.v1.Firestore")
        .disable_comments("google.firestore.v1.Firestore.RunAggregationQuery")
        .compile_with_config(
            config,
            &["proto/googleapis/google/firestore/v1/firestore.proto"],
            &["proto/googleapis"],
        )?;
    Ok(())
}
