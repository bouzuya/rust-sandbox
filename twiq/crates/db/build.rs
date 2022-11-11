use prost_build::Config;

fn main() {
    // brew install protobuf
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
        // service および rpc のコメント出力の無効化
        .disable_comments("google.firestore.v1.Firestore")
        .disable_comments("google.firestore.v1.Firestore.RunAggregationQuery")
        .compile_with_config(
            config,
            &["proto/googleapis/google/firestore/v1/firestore.proto"],
            &["proto/googleapis"],
        )
        .unwrap();
}
