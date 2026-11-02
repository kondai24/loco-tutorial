# Q&A サイト

このプロジェクトは、Udemy の『はじめての Ruby on Rails 入門-Ruby と Rails を基礎から学びウェブアプリケーションをネットに公開しよう』のコースを参考にしてハンズオン形式で作成した Q&A サイトです。

https://www.udemy.com/course/the-ultimate-ruby-on-rails-bootcamp/?couponCode=CM240325

## 学んだこと

- ruby の文法
  - ruby の基本的な文法事項
- rails の基礎
  - rails を用いた CRUD 処理の方法
  - MVC アーキテクチャについて

## React＋Loco版の起動方法

Railsを起動せず、Reactが画面、LocoがJSON API、SeaORMがSQLiteへの保存を担当します。質問CRUD、回答の投稿・削除、入力エラー表示に対応します。

### 準備

Rust（rustupのstable、検証時1.99.1）、Node.js（検証時24.21.0）、npmが必要です。

```bash
# Rustが未導入の場合（https://rustup.rs/）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup-init.sh
sh /tmp/rustup-init.sh -y --profile minimal
source "$HOME/.cargo/env"

# リポジトリのルートで実行
npm --prefix frontend ci
```

### Locoの開発ツール

公式チュートリアルと同じCLIを使用しています。モデル・エンティティを生成する開発者は次を実行します（今回の検証版へ固定）。

```bash
cargo install loco --version 1.2.0 --locked
cargo install sea-orm-cli --version 2.0.4 --locked
```

バックエンドは`loco new --db sqlite --bg blocking --assets none`で生成したLoco 1.2.0のAPIアプリが土台です。質問はscaffold、回答はmodel/controllerジェネレーターで生成後、Q&Aの仕様に合わせて編集しています。認証やサンプルワーカーは今回の対象から外しています。

### 起動（ターミナル2つ）

```bash
# ターミナル1: リポジトリルートから
cd backend
cargo loco start
```

```bash
# ターミナル2: リポジトリルートから
cd frontend
npm run dev
```

<http://127.0.0.1:5173> を開きます。APIは127.0.0.1:5150、画面は127.0.0.1:5173です。Viteが`/api`をAPIへ転送し、画面URLの直接アクセスにも対応します。終了はそれぞれCtrl+Cです。

初回起動で`backend/loco_development.sqlite`を新規作成し、migrationを適用します。既存Rails DBを移行・削除せず、再起動時もLocoのデータを維持します。RedisやDBサーバーは不要です。`DATABASE_URL`で別SQLiteへ変更できます。カレントディレクトリを`backend`にして起動してください。

### Loco標準の開発フロー

`backend/`で実行します。

```bash
cargo loco routes
cargo loco doctor
cargo loco db status
# migration追加後
cargo loco db migrate
cargo loco db entities
```

モデル追加は`cargo loco generate model`、コントローラ追加は`cargo loco generate controller`、まとめて作る場合は`cargo loco generate scaffold ... --no-auth`を利用します。新規migrationは`cargo loco generate migration`で追加し、適用済みmigrationは編集しません。詳細は[公式チュートリアル](https://loco.rs/docs/tutorials/your-first-app/)を参照してください。

- `src/models/_entities/`: DBからの生成物。手編集しません。
- `src/models/questions.rs`・`answers.rs`: 検索、保存、親子スコープ、検証、日時のフック。
- `src/controllers/`: 入力を取り出し、モデルを呼び、DTOを返す薄いハンドラー。
- `src/dtos/`: APIの入出力。`cargo test`で`frontend/src/bindings/`へTypeScript型を出力します。
- `tests/requests/`・`tests/models/`: Loco標準harnessでのAPI・モデルテスト。

`cargo loco`は生成された`.cargo/config.toml`のaliasで、`cargo run --`に相当します。Locoの標準エラー形式を使用し、検証失敗は400、存在しないレコードは404となります。JSONの型不一致はextractorによる422です。

### 検証

```bash
source "$HOME/.cargo/env"
rustup component add rustfmt clippy
cd backend
cargo fmt --all --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cd ../frontend
npm run build
npx playwright install chromium
npm run test:e2e
```

E2Eは5150/5173を使うため、通常起動中の両サーバーを停止してから実行します。テストが両サーバーを起動・終了し、APIテスト・E2Eとも専用のメモリSQLiteを使います。開発用DBへテストデータを保存しません。

`npm run build`は型検査とバンドル生成です。今回の配信経路はVite開発サーバーで、Locoからの静的ファイル配信・公開環境のデプロイは対象外です。

構成とAPIの契約は[ADR一覧](docs/adr/README.md)、APIの基本契約は[ADR 0004](docs/adr/0004-api-and-local-contract.md)、標準構成への統一と上書きした契約は[ADR 0005](docs/adr/0005-official-loco-conventions.md)を参照してください。Rails版のファイルは比較用として残しています。

## 旧Rails版の起動方法（比較用）

1. このリポジトリをクローンする．

```bash
git clone git@github.com:kondai24/rails-tutorial.git
```

2. 必要な Gem をインストールする．

```bash
bundle install
```

3. db のセットアップする

```bash
rails db:migrate
```

4. ローカルホストに立ち上げる．

```bash
rails s
```
