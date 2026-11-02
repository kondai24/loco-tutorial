# ADR 0001: ReactとLocoによる画面・APIの分離

- 日付: 2026-10-31
- 状態: 採用

## 背景

現行アプリケーションはRails 8のQ&Aサイトで、ERBによるHTML生成とTurbo／Stimulusを使用する。機能と現在に近いUIを保ちながら、バックエンドをRustのLocoへ移行する。フロントエンドをReactで書き直すことは合意済み。

## 決定

- 同一リポジトリ内で、React＋TypeScript＋Viteが画面表示、フォーム操作、画面遷移を担当する。
- Locoが`/api/`以下のJSON API、サーバー側の入力検証、質問・回答の操作を担当する。
- LocoからSeaORMを通じてSQLiteへアクセスする。
- ローカル開発ではViteとLocoを別プロセスで起動し、ブラウザからの`/api/`リクエストをViteがLocoへ転送する。
- 現在に近い見た目、文言、利用者の操作、画面URLを維持する。Turbo／Stimulusが担当していた画面上の役割はReactへ移す。
- 移行後の対象機能はRailsの起動に依存しない。

## 採用理由

画面とデータ処理の責務を分けられ、ERBの実行環境を残さずにLocoへ移行できる。ReactとLocoを同じリポジトリで管理し、画面とAPIの変更を対応付けられる。

## 非採用案

- ERBファイルを変更せずRailsを画面生成用に残す案: Railsへの実行時依存が残る。
- ERBをLocoのTeraテンプレートへ移植する案: 実現可能だが、今回はユーザーが許容し、合意したReact構成を採用する。
- フロントエンドを別リポジトリで管理する案: 今回のローカル移行では管理単位を増やさない。

## 影響と仕様化待ちの内容

HTMLフォームへのRails固有の応答とJSON APIの応答は同じ形式にはならない。利用者に見える機能を保ちつつ、APIの入力・出力・エラー形式、画面側のエラー表示と更新後の遷移を具体化する必要がある。依存ライブラリのバージョンや起動コマンドの詳細は未確定。

## 関連

- [ADR 0002](0002-local-parity-and-empty-sqlite.md)
- [ADR 0003](0003-decision-records.md)
- [Loco: React SPA](https://loco.rs/docs/how-to/build-a-spa/)
- [Loco: DBサポート](https://docs.rs/loco-rs/latest/loco_rs/db/index.html)

## 実装時の具体化

[ADR 0004](0004-api-and-local-contract.md)でAPI・データ・画面・ローカル検証の契約を具体化し、`backend/`と`frontend/`へ実装した。上記の仕様化待ち項目・静的調査の限界は設計時点の記録であり、現在の契約はADR 0004を参照する。
