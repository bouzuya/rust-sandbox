# iko

---

- [crates:sqlx] で動く DB マイグレーション用クレート
- sqlx migration との大きな違いは SQL ファイルではなく任意のコードを実行できる点にある
- 各マイグレーションは `u32` のバージョンと `async fn` の組である
- `sqlx::migrate::MigrationType` で言うと `Simple` に近いもののみを提供する
  - down 操作は提供できないことが多いため対応しない
  - エラー時には iko 外で個々に対応する必要がある
- `migration_status` テーブルで現在の DB マイグレーションの適用状況 (DB のバージョン) を管理する
- 適用済みのマイグレーションのすべてのバージョンを持つのではなく最新のバージョンのみを持つ
- エラー発生時には `in_progress` ステータスの行が残る

[crates:sqlx]: https://crates.io/crates/sqlx
