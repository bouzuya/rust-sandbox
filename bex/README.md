# bex

A client for Pocket (<https://getpocket.com>).

## Usage

```console
$ bex login
https://getpocket.com/auth/authorize?request_token=ffffffff-ffff-ffff-ffff-ffffff&redirect_uri=http://localhost:53039/
Logged in

$ bex status
Logged into xxxxxx-xxxxxxxxxxxxxxxxxxxxxxx

$ list=$(bex list --count 1)

$ echo "${list}" | jq .
[
  {
    "added_at": "2022-05-30T13:29:13Z",
    "id": "3629325863",
    "title": "2022-05-29 2022-W21 ふりかえり - blog.bouzuya.net",
    "url": "https://blog.bouzuya.net/2022/05/29/"
  }
]

$ echo "${list}" | jq -r '.[] | .url' | xargs open

$ echo "${list}" | jq -r '.[] | .id' | xargs bex delete
Deleted 3629325863

$ bex logout
Logged out
```

## TODO

- v0.1.0:
  - ☑ `bex login`
  - ☑ `bex logout`
  - ☑ `bex list`
    - ☑ `--count <COUNT>`
    - ☑ `--tag <TAG>` ... タグなしのみは `--tag '_untagged_'`
  - ☑ `bex delete <ID>`
- v0.2.0:
  - ☑ `bex status`
- v0.x.0:
  - ☐ offline cache

## メモ

- Pocket <https://getpocket.com> から情報を得る
- Pocket Developer API
  <https://getpocket.com/developer/>
- Pocket API: Retrieving a User's Pocket Data
  <https://getpocket.com/developer/docs/v3/retrieve>
- 背景: Pocket がある種のゴミ箱と化している
  - 「あとで読む」と言い訳をしながら捨てられることに精神面での利点はありそうだが先はない
  - 「ゴミ箱」にならないよう未読を活用できると良さそう
- パソコン・スマホから横断的に URL を集める場所として Pocket を使う
  (ブラウザ拡張・スマホアプリの開発コストを避けられる)
- bex は Pocket の未読を減らすためのもの
  - 追加なし・削除 (既読化) のみ
  - 汎用的なクライアントをつくっても面白みがない
- bex は日常的なコマンドの合間に未読の情報を「埋め込む」ために使用するので CLI
  - パソコンなら Pocket のサイトを使えばいい
    - "View Original" と言いつつクエリ文字列に `utm_source=pocket_mylist` を追加する不快極まりない仕様はある
    - 一覧性が悪く UI もイマイチ
    - 他のユーザーとのつながり……要る？
  - スマホなら Pocket のアプリを使えばいい
    - リストの高さが一定じゃない時点で目が滑ってつらい
