use std::{fs, path::Path};

use prost_build::Config;

fn main() {
    // brew install protobuf
    let out_dir = Path::new("src/firestore_rpc");
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }
    fs::create_dir_all(&out_dir).unwrap();
    let mut config = Config::new();
    config.disable_comments(&[
        // message や field は消せるが service や rpc は消せない
        // 前者は prost-build crate の範囲、後者は tonic-build crate の範囲
        "google.api.HttpRule",
        "google.firestore.v1.StructuredQuery.start_at",
        "google.firestore.v1.StructuredAggregationQuery.Aggregation.alias",
        "google.firestore.v1.StructuredAggregationQuery.Aggregation.Count.up_to",
    ]);
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        // service および rpc の comments 出力の廃止
        .disable_comments("google.firestore.v1.Firestore")
        .disable_comments("google.firestore.v1.Firestore.RunAggregationQuery")
        .compile_with_config(
            config,
            &["proto/googleapis/google/firestore/v1/firestore.proto"],
            &["proto/googleapis"],
        )
        .unwrap();
}
