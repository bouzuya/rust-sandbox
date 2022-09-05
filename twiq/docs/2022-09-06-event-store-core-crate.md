# event_store_core crate の導入

理想。 domain crate で EventStreamId を使用したい。
集約の生成時に EventStreamId を生成したい。

現状。 EventStreamId は db crate で定義されている。
db crate は永続化を担っており、永続化の詳細を知っている。

課題。 domain crate から db crate へ依存することは想定する依存の向きと逆になってしまう。

解決策。この依存の向きが逆になる問題を its 0.21.0 (<https://github.com/bouzuya/rust-sandbox/tree/its/0.21.0/its>) では次のように解決している。

- domain で生成する Event には EventStreamId を含まない
  <https://github.com/bouzuya/rust-sandbox/blob/its/0.21.0/its/crates/domain/src/domain/event/issue_created.rs#L5-L11>
- 永続化するタイミングで EventStreamId を生成する
  <https://github.com/bouzuya/rust-sandbox/blob/its/0.21.0/its/crates/adapter_sqlite/src/adapter/sqlite/sqlite_issue_repository.rs#L210-L222>
- EventStreamId と AggregateId (ここでは IssueId) を対応づけるテーブルで管理する
  <https://github.com/bouzuya/rust-sandbox/blob/its/0.21.0/its/crates/adapter_sqlite/src/adapter/sqlite/sqlite_issue_repository.rs#L223-L224>
- 各 EventStore の操作時に AggregateId から EventStreamId を取得する
  <https://github.com/bouzuya/rust-sandbox/blob/its/0.21.0/its/crates/adapter_sqlite/src/adapter/sqlite/sqlite_issue_repository.rs#L188-L190>

別の解決策。 AggregateId と EventStreamId の値を同一にする。

- Pros: 読み替え用のテーブルが不要になり、実装を簡素化できる
- Cons: 実装の詳細 (UUIDv4) に依存する
- Cons: 集約と Event Stream の対応関係を変えることが難しくなる
- Cons: AggregateId に「集約の種類ごとで一意」ではなく「集約全体で一意」であることが要求される
- Cons: すべての AggregateId に同じ実装 (UUIDv4) を採用することが要求される

気になる点は読み替え用のテーブル (≒検索インデックス用のテーブル) は EventStreamId で不要だったとしても「名前 (などの別の属性) で検索」などの要件が発生すると必要になる。

読み替え用テーブルではなく events テーブル自体に都度インデックス用の列を追加する方法もある。

今回の Firestore に最適化させてしまうなら events コレクションに data 属性に JSON で持たせるのではなくスキーマレスを活かして展開した形で持たせる方法もある。

できるだけ特定の技術に依存しない形で実装したいと考えているので避けたい。

今回の Firestore で考えると読み替え用テーブルを追加する場合は同一トランザクションで処理するために現状の db (EventStore) の API を改めないといけない。

(WIP)
