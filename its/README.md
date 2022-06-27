# its

its: issue tracking system

課題追跡システム

## TODO

- ☑ command: ある issue を延期する (期限を更新する)
- ☑ command: ある issue を追加する
- ☑ command: ある issue を完了する
- ☑ query: ある日の issue を一覧する
  - タイトル
  - 完了状況 (未完了・完了)
  - 期限
- feature: issue block link
- ☑ command: ある issue が block している issue を指定して issue link を作成する
- ☑ command: ある issue link を削除する
- ☑ query: ある issue が block している issue の一覧を取得する (A blocks X)
- ☑ query: ある issue を block している issue の一覧を取得する (A is blocked by X)
- ☐ feature: restore query db
- ☑ command: ある issue のタイトルを変更する
- ☐ command: ある issue の詳細 (?) を追加する

## メモ

### やりたいこと / UseCase の戻り値から情報を減らしたい

- UseCase の戻り値から情報を減らしたい
- `impl From<IssueManagementContextEvent> for DomainEvent` を削除したい
  → 2022-06-12 done
- update_query_db が `impl From<IssueManagementContextEvent> for DomainEvent` を使用している
  → 2022-06-12 done
- (脱線) query db は EventStore と Event に依存していて Command db に依存するわけではない
- (脱線) command db から index として使用しているテーブルと event store との関係を切る

### 完了: やりたいこと / `its issue finish` の表示を改善したい

- `its issue finish` の表示を改善したい
  → 2022-06-12 done
- `finish_issue` はコマンドであってクエリではない
- CLI としての利便性を考えるとコマンド→クエリを自動で実行してくれると良い
- JSON で Issue の状態を返してほしい
- もしかするとその操作の詳細がほしい……かもしれない (EventId を指定して取得)
- もしかするとその操作の直後の状態がほしい……かもしれない (AggregateVersion を指定して取得)
- 現状は UseCase から返された `Vec<IssueManagementContextEvent>` を `{:?}` で表示している
- AggregateId + AggregateVersion があれば任意の状態を復元できるはずだが query db は最新状態にしか対応していなさそう
- ひとまず「 UseCase からの戻り値は EventId および AggregateId を含める」「 AggregateId で最新の状態を得て表示する」で良さそう
- (脱線) 現状は AggregateId ≒ EventStreamId (1 対 1) になっている

### 迷い / UseCase から何を返すべきか

- UseCase から何を返すべきかに迷いがある
- UseCase の戻り値は詳細な情報ではなく情報のキーを返すほうが良いのではないか？
- 現状は `pub struct IssueManagementContextEvent(DomainEvent);` で `DomainEvent` と 1 対 1 。
- 少なくとも永続化のためのイベントではなく他のコンテキストに提供すべき情報になるはず

### events.seq がある理由

- `events.seq` がある理由は何か
- query db の構築の際などですべての events を発生順に読みたい
- これを `event_store::find_events_by_event_id_after` や `find_events` で提供する
- Event に at や id (ULID) はあるが AP 側で作成するため信用できない時刻である
- DB 側で作成する連番・時刻が必要だ
- 今回は `events.seq` として連番を追加する
- これは AP 側には返さず DB 側のみで管理する

### events.id がある理由

- `events.id` がある理由は何か
- `events.event_stream_id` + `events.version` (=event_stream_seq) で指定するのがわずらわしいから
- EventStream によらず Event 全体を対象に操作する場合に EventStreamId + EventStreamSeq だと扱いづらいため

### Command 側および Query 側が EventStore と (Domain) Event に依存する理由

- command 側の model に query 側が引きずられるのは良くないと判断したため
- query 側を作成する際に command 側の model に追加したくなったり、その逆を避けるため
- domain event は aggregate ごとに区切られているので command への依存を切るのはすこし怪しい
