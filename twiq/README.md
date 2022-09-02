# twiq

自身のツイートを検索する Web アプリケーション。

## TODO

- 設定に従ってツイートを収集する
- 収集したツイートから検索する

## メモ

- 何のためにつくるのか
  - Cloud Firestore の検証
  - 自身のツイートのバックアップの提供
  - 過去のツイートを検索する CLI の提供
    (twilog.org の代替)
- どのようにつくるのか
  - 全体像は構成図を参照
  - 詳細は未定
- 構成要素ごとの役割
  - cli は web に検索クエリを発行する
  - db は情報を保持する
  - scheduler は定期的に worker を動かす
  - web は cli からの呼び出しに応じて db を読み込んで返す
  - worker は twitter から情報を読み込み db に書き込む

## 構成図

```mermaid
graph LR
  bouzuya
  subgraph twiq
    subgraph client
      cli
    end
    subgraph server["server (GCP)"]
      web["web (Cloud Run)"]
      worker["worker (Cloud Run)"]
      db["db (Cloud Firestore)"]
      scheduler["scheduler (Cloud Scheduler)"]
      scheduler --> worker
      web -- read --> db
      worker -- write --> db
    end
  end
  twitter
  bouzuya --> cli
  cli --> web
  worker -- read --> twitter
```

## user の解決

- user の id から name などの情報を取得する

### user の解決 / 構成図

```mermaid
graph LR
  user

  subgraph foreground
    web
    aggregate["user aggregate"]
    repository["user repository"]
  end
  subgraph background
    scheduler
    qdbworker["query db worker"]
    requestworker["request worker"]
  end
  subgraph store
    qdb["query db"]
    eventstore["event store (command db)"]
  end
  twitter

  user -- request --> web
  web -- read --> qdb
  web -- command --> aggregate
  web -- load/store --> repository
  qdbworker -- read --> eventstore
  qdbworker -- read/write --> qdb
  repository -- read/write --> eventstore
  repository -- read --> aggregate
  requestworker -- read/write --> eventstore
  requestworker -- read --> twitter
  scheduler --> qdbworker
  scheduler --> requestworker
```


### user の解決 / システム外部へのインタフェース

- `GET /users/:id`
  - status code: 200 OK or 202 Accepted
  - `{"id":"...","name":"...","updated_at":"..."}`
  - 内部処理:
    - 取得済みの情報が query db にあれば 200 でそれを返す、なければ 202 を返す
    - 取得済みの情報がないか updated_at が 86400s よりも前なら次の処理をする
      - user 集約を得る、得られなければ作成する
      - user 集約に取得要求する
      - user 集約を保存する

### user の解決 / user 集約

```mermaid
stateDiagram-v2
  [*] --> 要求待ち: 作成
  要求待ち --> 取得待ち: 取得要求
  取得待ち --> 要求待ち: 取得結果反映
```

- 状態:
  - 要求待ち
  - 取得待ち
- 操作:
  - 作成 (user_id)
    - `[*] --> 要求待ち`
  - 取得要求
    - `要求待ち --> 取得待ち`
  - 取得結果反映 (status_code, body)
    - `取得待ち --> 要求待ち`
- 追加 DB:
  - users(user_id (PK), event_stream_id (UK))
    - user_id → event_stream_id の解決のための index
    - event_stream_id → user_id の解決も可能

### user の解決 / query db worker

- ...
- 追加 DB:
  - user_query_db(last_event_id)

### user の解決 / request worker

- ...
- 追加 DB:
  - user_requests(last_event_id)
