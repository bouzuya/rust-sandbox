# b

b is a CLI for taking notes.

## Installation

```console
$ version=0.10.0
$ curl -L "https://github.com/bouzuya/rust-sandbox/releases/download/b%2F${version}/b-x86_64-apple-darwin" > b
$ chmod +x b
```

## メモ

- b の query と bbn の query は別物
  - b は最小単位が時間 bbn は最小単位が日
    - 例えば `date:2021-02-03` としたとき
      - b は指定したタイムゾーンにおける時間の範囲で読み込む
      - bbn はその日だけを読み込む
