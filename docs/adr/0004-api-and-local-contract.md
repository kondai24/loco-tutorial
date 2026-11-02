# ADR 0004: ローカル移行のAPI・データ・画面契約

- 日付: 2026-10-31
- 状態: 採用（フレームワーク版・エラー契約・ID/日時型・構成はADR 0005で一部置換）

## 背景

ADR 0001・0002を実装できる契約に具体化する。実装依頼に基づき、現行のコードと設計で未確定だった細部を以下のように定める。

## 2026-11-01の更新

[ADR 0005](0005-official-loco-conventions.md)に従い、Loco 1.2.0・SeaORM 2.0系、生成エンティティ、モデルでの検証、標準エラーへ統一した。以下は初版の契約であり、検証失敗は現在400、JSON型不一致は422、エラーはLoco標準形式、IDはi64、日時は生成スキーマのtimezone付き日時型となる。質問・回答の操作と成功時のJSONエンベロープ、画面URLは維持する。

## 決定

### API

JSONの入力はトップレベルのオブジェクトで、Railsの`question`・`answer`ラッパーを使用しない。

| メソッド・経路 | 入力 | 成功 |
| --- | --- | --- |
| GET /api/questions | なし | 200 `{questions: Question[]}` |
| GET /api/questions/:id | なし | 200 `{question: Question, answers: Answer[]}` |
| POST /api/questions | title, name, content: string | 201 `{question: Question}`、Locationヘッダ |
| PUT /api/questions/:id | title, name, content: string | 200 `{question: Question}` |
| DELETE /api/questions/:id | なし | 204、本文なし |
| POST /api/questions/:id/answers | name, content: string または null（省略可） | 201 `{answer: Answer}` |
| DELETE /api/questions/:question_id/answers/:id | なし | 204、本文なし |

- 質問の更新はフォームの全入力項目を送るPUTとし、部分更新は追加しない。
- 質問の入力省略は空文字として検証する。nullや文字列以外の型、不正JSONは400。未知の入力項目は保存しない。
- 検証失敗は422 `{error: "validation_failed", errors: string[]}`。
- 不明な質問・回答、別質問に属する回答の削除は404 `{error: "not_found", message: string}`。
- DB失敗は500 `{error: "internal_error", message: string}`。詳細はサーバーログのみ。
- JSON抽出失敗は400 `{error: "invalid_request", message: string}`。整数に変換できないパスや未知ルートなど、ルーターによる拒否にはフレームワークの応答を使用する。
- 一覧と回答はID昇順。ページ分割は今回追加しない。

### データ・検証

- 質問: id（SQLite整数、自動採番、APIはi32）、title/name/content（TEXT NOT NULL）、created_at/updated_at（UTCのRFC3339文字列）。
- 回答: id、question_id（NOT NULL外部キー＋インデックス）、name/content（NULL可）、同形式の日時。
- 質問はtitle/name/contentが空またはUnicode空白のみなら不正。本文の長さはRustの`chars().count()`でUnicodeスカラー値を数え、5以上。前後空白は保存時に除去しない。結合文字や絵文字列の見た目の文字数とは異なる。
- 回答は空文字、NULL、省略を許し、必須・長さ制約を追加しない。
- 質問削除はSQLiteの`ON DELETE CASCADE`で回答を同じDELETE文の原子性の下で削除する。SeaORM/SQLxのSQLite接続は外部キー有効。実DBで検証する。
- 別ファイル`backend/loco_development.sqlite`から開始する。RailsのDBは読み書きしない。起動時は未適用migrationのみ実行し、既存データのtruncate/recreateは行わない。

### 画面・開発構成

- React＋TypeScript＋Viteを`frontend/`、Loco 0.16.4＋SeaORM 1.1系を`backend/`へ配置。再現バージョンはCargo.lock/package-lock.jsonに固定する。
- `/`、`/questions`、`/questions/new`、`/questions/:id`、`/questions/:id/edit`を維持。表示は現行の英語見出し・ブラウザ標準に近い外観を使用。
- 5173のViteが5150のLocoへ`/api`を転送。両プロセスはループバックにバインド。画面URLの直接アクセス・再読込はViteのSPA fallbackが処理する。
- 新規・更新後は詳細へ、質問削除後は一覧再取得、回答操作後は詳細再取得。削除前に確認する。
- 読み込み表示、送信中のボタン無効化、入力内容を維持したエラー表示、取得失敗時の再試行を用意。通信・HTTPエラーでも入力は保持する。
- Rails資産は比較用に保持する。移行後の起動・操作はRails、Redis、認証に依存しない。公開デプロイは対象外。

### 受入確認

- 実SQLiteのAPIテストで空一覧、質問作成・取得・更新・削除、Unicode/空白/4文字境界、入力型、任意回答、親子スコープ、連鎖削除、404、DB失敗を確認。
- Playwrightで質問CRUD・検証修正・回答投稿削除・確認キャンセル・直接URL/再読込・通信失敗を確認。
- Rustのformat/clippy、フロントの型検査とビルドを実施する。

## 理由・代替案・影響

全項目フォームに合わせたPUT、DBの外部キー、単純なJSON契約を採用し、移行対象を小さく保つ。RailsのHTML応答そのものやActiveRecordのコールバックを再現する方式は採用しない。将来削除時の副作用を追加する場合はDB cascadeだけで十分かを再検討する。

- [ADR 0001](0001-react-loco-architecture.md)
- [ADR 0002](0002-local-parity-and-empty-sqlite.md)
- [Loco 0.16.4公式ソース](https://github.com/loco-rs/loco/tree/v0.16.4)
