# crate 間の依存関係 2022-10-17

```mermaid
graph LR
  db
  domain
  event_store_core
  import_twitter_data
  query_handler
  twitter_user_id
  use_case
  web
  worker

  db --> domain
  db --> event_store_core
  db --> use_case
  db -. "追加予定" .-> query_handler
  domain --> event_store_core
  import_twitter_data --> domain
  query_handler --> domain
  use_case --> domain
  use_case --> event_store_core
  web --> db
  web --> domain
  web --> query_handler
  web --> use_case
  web -- "削除予定" --> worker
  worker -. "追加予定" .-> db
  worker --> domain
  worker --> event_store_core
  worker --> query_handler
  worker --> use_case
```

## 気になる点

- `web --> domain` は削除できないか
  - `domain::event` を使用しているため不可
- `worker --> domain` は削除できないか
  - `domain::event` を使用しているため不可
- `worker --> event_store_core` は削除できないか
  - `WorkerRepository` が使用しているため不可
  - `InMemoryWorkerRepository` が使用しているため不可
- `use_case` は `command_handler` にリネームしても良さそう
