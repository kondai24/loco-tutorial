# Q&A サイト

このプロジェクトは、Udemy の『はじめての Ruby on Rails 入門-Ruby と Rails を基礎から学びウェブアプリケーションをネットに公開しよう』のコースを参考にしてハンズオン形式で作成した Q&A サイトです。

https://www.udemy.com/course/the-ultimate-ruby-on-rails-bootcamp/?couponCode=CM240325

## 学んだこと

- ruby の文法
  - ruby の基本的な文法事項
- rails の基礎
  - rails を用いた CRUD 処理の方法
  - MVC アーキテクチャについて

## Q&A アプリケーションの起動方法

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
