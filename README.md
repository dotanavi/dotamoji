## 概要

形態素解析を行うためのツールの予定です

## 使い方

### Rustをインストール

https://rustup.rs/ に従い、Rustをインストールしてください。
自動的に最新の安定版がインストールされます。

### ビルド

以下のコマンドで最適化したバイナリを作成します。
初回はダウンロードや依存ライブラリのビルドを含むので時間がかかります。

```
cargo build --release
```

ビルド結果は `./target/release/` に保存されます。必要に応じてコピーしてください。

### 辞書の構築

入力ファイルをダブル配列にして、出力ファイルに保存します。
現在単語の登録のみを行っており、ここからコストなどを取り出すことはできません。

```
./target/release/build-dict array [出力ファイル] < [辞書テキスト]
```

ダブル配列の代わりに、再帰ハッシュマップで構築した辞書も作成できます。

```
./target/release/build-dict hash [出力ファイル] < [辞書テキスト]
```

### 確認

出力した辞書に全単語が載っているかをチェックします。

```
./target/release/test-dict array [出力された辞書ファイル] < [辞書テキスト]
```
