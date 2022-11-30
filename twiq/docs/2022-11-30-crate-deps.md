# crate 間の依存関係 2022-11-30

```mermaid
graph LR
  command_handler
  db
  domain
  event_store_core
  import_twitter_data
  job
  query_handler
  twitter_user_id
  web
  worker

  ENTRYPOINT --> import_twitter_data
  ENTRYPOINT --> job
  ENTRYPOINT --> twitter_user_id
  ENTRYPOINT --> web
  command_handler --> domain
  command_handler --> event_store_core
  db --> command_handler
  db --> domain
  db --> event_store_core
  db --> query_handler
  db --> worker
  domain --> event_store_core
  import_twitter_data --> domain
  job --> command_handler
  job --> db
  job --> query_handler
  job --> worker
  query_handler --> domain
  web --> command_handler
  web --> db
  web --> domain
  web --> query_handler
  web --> worker
  worker --> command_handler
  worker --> domain
  worker --> event_store_core
  worker --> query_handler
```

## 気になる点

- `web --> domain` は削除できそう
  - `domain::event` を使用しているため不可→ `command_handler` / `worker` で隠せそう
- `job --> domain` は削除できそう
  - `domain::event` を使用しているため不可→ `command_handler` / `worker` で隠せそう
- `worker` は `command_handler` に統合しても良さそう
  - `query_handler` にも依存しているため不可
