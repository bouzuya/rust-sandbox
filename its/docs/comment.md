# コメント機能

## メモ

### Issue についているコメントとはなにか

- 例を挙げてみる
- 例「変更理由」
- 例「 Issue に関連した議論」
- 例「任意のテキストによる追加の情報」

### IssueComment は値かエンティティか

- IssueComment というエンティティを考える
- 同一のテキストメッセージでも異なる人からの異なる時間に投稿されたものかもしれない
- そういったものを組として値とする手もあるが別の Id を持つエンティティとする
- 仮に IssueComment はエンティティとする

### IssueComment は集約か

- IssueComment は集約かを考える
- IssueComment が集約ではない状況としてたとえば Issue と IssueComment は同一集約かを考える
  - これらは更新する上で不可分か
  - 同一でないといけない制約があるか
- 例「変更理由」を考えると「変更」と「変更理由」は不可分かも？
  - しかし Git では変更とコミット(メッセージ)は別
- 「変更」と「変更理由」が不可分だとして「変更理由」だけを修正したい場合はどうなる？
  - 「特定の「「変更」と「変更理由」」の組のうち「変更理由」が修正されたというイベント」を追加する？
  - 「変更」部分を修正できない点に違和感がある ( ……が、できたら大変なことになる )
- 「変更」と「変更理由」は不可分ではないとする
- 「変更理由」から特定の「変更」を指すことができると良いのかもしれない
  - ほとんど場合は時系列で直前のものを指すと思うので必須ではなさそう
- 制約は思いつかない
  - IssueComment は特定の Issue に対してつくので IssueId を持ちそう
  - 対象の Issue なしに Comment だけで良いなら IssueComment ではなさそう
- Issue と IssueComment は不可分ではない、とする
- 仮に IssueComment は集約とする

### IssueComment の属性は何か

- IssueComment の属性を考える
- id: IssueCommentId …… Id
- issue_id: IssueId …… 対象の Issue
- message: String …… テキストメッセージ
- (投稿者 …… ITS はシングルユーザーなので不要)
- at: Instant …… 投稿日時

### IssueComment の操作・イベントは何か

- IssueComment の操作・イベントは何か
- command: CreateIssueComment
  - issue_id: IssueId
  - message: String
- event: IssueCommentCreated
  - id: IssueCommentId (生成)
  - issue_id: IssueId
  - message: String
  - at: Instant (取得)
- command: UpdateIssueComment
  - id: IssueCommentId
  - message: String
- event: IssueCommentUpdated
  - id: IssueCommentId
  - message: String
  - at: Instant (取得)
- command: DeleteIssueComment
  - id: IssueCommentId
- event: IssueCommentDeleted
  - id: IssueCommentId
  - at: Instant (取得)
