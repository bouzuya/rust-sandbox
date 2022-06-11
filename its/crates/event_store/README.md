# event_store

[crates:sqlx] の Any driver を前提とした EventStore の操作を集めたもの。

(現状は sqlite でしか検証されていない)

以下の 2 テーブルが存在することを前提としている。

```sql
CREATE TABLE IF NOT EXISTS event_streams (
    id      CHAR(26) NOT NULL,
    version INTEGER  NOT NULL,

    CONSTRAINT event_streams_pk PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS events (
    seq             INTEGER  PRIMARY KEY AUTOINCREMENT,
    id              CHAR(26) NOT NULL,
    event_stream_id CHAR(26) NOT NULL,
    version         INTEGER  NOT NULL,
    data            TEXT     NOT NULL,

    CONSTRAINT events_uk1 UNIQUE (id),
    CONSTRAINT events_uk2 UNIQUE (event_stream_id, version),
    CONSTRAINT events_fk1 FOREIGN KEY (event_stream_id) REFERENCES event_streams (id)
);
```

## TODO

- events.seq より events.at のほうが良いかもしれない
  - seq は RDBMS の実装に引きずられがちなので
- events.version は events.event_stream_seq としたい
- event_streams.version は event_streams.seq としたい
- event_streams.type を追加したほうが良いかもしれない
  - event_streams 内のグループ化のため
  - 実用上は「どの集約か」という判定を提供するため
  - 現状は次の理由から追加していない
    - Event から「どの集約か」は分かる
    - Command DB に集約 ID と EventStreamId の対応付けを持っている

## 参考

<https://cqrs.files.wordpress.com/2010/11/cqrs_documents.pdf>
<https://www.minato.tv/cqrs/cqrs_documents_jp.pdf>

```sql
CREATE TABLE Aggregates(
  AggregateId Guid,
  Type        Varchar,
  Version     Int,
  PRIMARY KEY(AggregateId)
);
CREATE TABLE Events(
  AggregateId Guid,
  Data        Blob,
  Version     Int,
  PRIMARY KEY(AggregateId, Version)
);
```

[crates:sqlx]: https://crates.io/crates/sqlx
