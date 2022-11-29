# crate 間の依存関係 2022-11-30

```mermaid
graph LR
  db
  domain
  event_store_core
  import_twitter_data
  job
  query_handler
  twitter_user_id
  use_case
  web
  worker

  db --> domain
  db --> event_store_core
  db --> query_handler
  db --> use_case
  db --> worker
  domain --> event_store_core
  import_twitter_data --> domain
  job --> db
  job --> query_handler
  job --> use_case
  job --> worker
  query_handler --> domain
  use_case --> domain
  use_case --> event_store_core
  web --> db
  web --> domain
  web --> query_handler
  web --> use_case
  web --> worker
  worker --> domain
  worker --> event_store_core
  worker --> query_handler
  worker --> use_case
```

## 気になる点

- `web --> domain` は削除できそう
  - `domain::event` を使用しているため不可→ `use_case` / `worker` で隠せそう
- `job --> domain` は削除できそう
  - `domain::event` を使用しているため不可→ `use_case` / `worker` で隠せそう
- `worker --> domain` は削除できないか
  - `domain::event` を使用しているため不可
- `use_case` は `command_handler` にリネームしても良さそう
- `worker` は `use_case` に統合しても良さそう
