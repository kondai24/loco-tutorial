# ADR 0005: 公式ジェネレーターとLoco標準の構成への統一

- 日付: 2026-11-01
- 状態: 採用
- 一部置換: [ADR 0004](0004-api-and-local-contract.md)のフレームワーク版・実装構成・エラー契約・型

## 背景

ユーザーから公式のYour First Appを参照し、基本的なLocoの書き方へ統一する依頼があった。初版は0.16.4のAPIを用いて構成・エンティティ・エラー処理を手書きしており、公式のgenerate→editとモデル中心の構成を使っていなかった。

## 決定

- 公式ドキュメントと一致するLoco 1.2.0、SeaORM 2.0系を使用する。
- `cargo install loco --locked`、`cargo install sea-orm-cli --locked`を実行し、`loco new`で生成したAPIアプリを土台にする。
- `cargo loco generate scaffold questions ... --no-auth`、`cargo loco generate model answers ... question:references`、`cargo loco generate controller answers`で必要なファイルと配線を生成してから編集する。
- `backend/.cargo/config.toml`の公式aliasを利用し、起動は`cargo loco start`、DB操作は`cargo loco db migrate/entities/status`とする。
- `src/models/_entities/`はDBから生成し、手編集しない。独自の検索・作成・状態変更は`src/models/questions.rs`と`answers.rs`へ置く。
- 検証はモデルの`Validatable`、`validator::Validate`、`ActiveModelBehavior::before_save`で表現する。質問保存の入口でも型付き検証エラーを伝播し、HTTPを通らない保存もbefore_saveで保護する。
- 日時は生成スキーマのtimezone付き日時型、IDは生成標準のi64に揃える。日時の初期値・更新はモデルのフックが担当し、DTOは日時を文字列としてJSON化する。
- コントローラは`loco_rs::prelude::*`、`#[debug_handler]`、`Result<Response>`、`format::json`/`format::render`を用い、モデルの操作とDTOの応答に絞る。
- 独自ApiErrorを廃止する。LocoのModelError::EntityNotFoundは404、Validationは400、DB障害は500。エラーボディはLoco標準のerror/description/errorsを利用する。JSON抽出時の型不一致・不正JSONもフレームワークの応答に従う。
- レコード自体をJSONへ直列化せず、生成DTOを編集して明示的な応答契約にする。ts-rsでTypeScript型を出力し、Reactから参照する。
- テストは`tests/requests/`・`tests/models/`とLoco標準のrequest/boot_test harnessへ移す。
- 質問CRUD・回答の投稿削除・空白とUnicode検証・親子スコープ・連鎖削除・画面URLは維持する。一覧の全件表示は既存契約を維持し、ページ分割機能は追加しない。
- 生成スターターの認証・メール・サンプルタスク/ワーカーは対象外として除く。SQLiteのみ必要なのでdb-sqliteを有効化し、Redis等は必要としない。
- backend/frontendという同一リポジトリ内の分離は維持。Viteで/apiを転送し、公開配信は今回も対象外。

## 理由・代替案・影響

aliasだけを追加して旧実装を残す案は、モデル・検証・生成物管理まで揃える要望を満たさないため採用しない。公式スターターにある認証機能を利用者向けに追加する案も元の機能範囲を超えるため採用しない。

初版の開発用DBファイルが作業開始時点で存在しないことを確認し、未配布の初期migrationを公式生成のmigrationへ置換する。旧Rails DBは変更しない。将来このスキーマを実データ環境へ適用した後は、適用済みmigrationを編集せず、新規migrationを追加する。

検証ステータスとエラー形式の変更にReactとAPIテストを追従させる。受入条件はAPI・モデルテスト、既存E2E、format/clippy、フロント型検査・ビルドが成功すること。

## 一次資料

- [Your First App](https://loco.rs/docs/tutorials/your-first-app/)
- [Add a model](https://loco.rs/docs/how-to/add-model/)
- [Add a controller](https://loco.rs/docs/how-to/add-controller/)
- [Validate requests](https://loco.rs/docs/how-to/validate-requests/)
- [Loco 1.2.0の開発ガイド](https://github.com/loco-rs/loco/blob/v1.2.0/skills/loco/doctrine.md)
