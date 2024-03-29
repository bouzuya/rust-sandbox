//! Aggregates
//!
//! 集約 (aggregate) は……
//!
//! - ドメインレイヤーの外部に公開する API である
//! - 次のものを提供する
//!   - 集約
//!   - 集約イベント
//!   - 集約エラー
//!   - 値オブジェクト (集約コマンド・集約イベントで使用されるもののみ)
//! - 集約はトランザクション関連関数を持つ
//! - トランザクション関連関数は次の特徴を持つ
//!   - 強い整合性 (DB トランザクション) の単位である
//!     - DB トランザクションの制御自体は外部に委ねる
//!     - 単一のトランザクションで実行されることを想定する
//!   - 同一の引数からは一定の戻り値を返す
//!   - トレイト経由を含む副作用のある操作を含まない
//! - リポジトリによる I/O の単位である
//! - 集約イベントは変更後の aggregate を含まない
//! - 集約イベントを集約外部で構築されるのは良くないように思う
//!   - 集約イベントは集約のみが構築 (発行) できることが理想である
//!   - 永続化の観点から完全に隠すのは不可能である
//!   - 構築の危険性を伝える関数名にしておく
//!     - from_trusted_data(...)
//! - 集約イベントはドメインイベントのうちその集約から発生するものだけを取り出してまとめたもの
//!   - TODO: ドメインイベントとは
//! - TODO: I/O のための DTO の考慮
//! - 重要な点として一般的には集約は型を持つわけではないのだけど、このプロジェクトでは別にしている
//!   - entity は変更履歴ではなく現在の状態のみに集中する
//!   - 集約は command と event による API を提供する
//! - Old: 集約コマンドは廃止された
//!   - 対象の集約を Command に含めるよりも `aggregate.command()` のほうが簡潔になるため
//!   - create / update を分離するだけでなく `Aggregate::transaction` を廃止した
pub mod issue;
pub mod issue_block_link;
pub mod issue_comment;

pub use self::issue::{IssueAggregate, IssueAggregateEvent};
pub use self::issue_block_link::{IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent};
