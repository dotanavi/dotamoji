[![Build Status](https://travis-ci.org/travis-ci/travis-web.svg?branch=master)](https://travis-ci.org/travis-ci/travis-web)

## 概要

形態素解析を行うためのツールです

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

```
./target/release/build-dict array16 [出力ファイル] < [辞書テキスト]
```

ダブル配列以外の形式にも対応しています。

- `array16`: ダブル配列
- `hash16`: 再帰ハッシュマップ
- `trie16`: トライ木
- `trans16`: トライ木でパース、ダブル配列として保存

```
./target/release/build-dict [変換オプション] [出力ファイル] < [辞書テキスト]
```

### 確認

出力した辞書に全単語が載っているかをチェックします。

```
./target/release/test-dict array16 [出力された辞書ファイル] < [辞書テキスト]
```

`array16` の他に、 `hash16`, `trie16` が利用できます。

### 形態素解析

標準入力を形態素解析します。

```
echo "すもももももももものうち" | ./target/release/analyze array16 [出力された辞書ファイル] [連結コストファイル]
```

`array16` の他に、 `hash16`, `trie16` が利用できます。

