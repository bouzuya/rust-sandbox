# stamp-rally

## モジュール間の依存関係

```dot
digraph {
  rankdir = "BT"
  ports -> entities
  use_cases -> entities
  use_cases -> ports
  adapters -> entities
  adapters -> ports
  application -> entities
  application -> use_cases
  application -> adapters
  application -> ports
}
```

## ユースケース

- 参加する (join)
  - あるユーザー (User) はあるスタンプラリー (StampRally) に参加できる (join)
  - 参加することでユーザー (User) は参加者 (Player) を得られる
