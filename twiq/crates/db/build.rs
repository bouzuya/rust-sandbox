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
        "google.api.HttpRule",
        "google.firestore.v1.StructuredQuery.start_at",
    ]);
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        .compile_with_config(
            config,
            &["proto/googleapis/google/firestore/v1/firestore.proto"],
            &["proto/googleapis"],
        )
        .unwrap();
}
