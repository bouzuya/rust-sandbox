use std::{fs, path::Path};

fn main() {
    // brew install protobuf
    let out_dir = Path::new("src/firestore_rpc/gen");
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }
    fs::create_dir_all(&out_dir).unwrap();
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/firestore_rpc/gen")
        .compile(
            &["googleapis/google/firestore/v1/firestore.proto"],
            &["googleapis/"],
        )
        .unwrap();
}
