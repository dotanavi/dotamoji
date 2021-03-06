[![Build Status](https://travis-ci.org/dotanavi/dotamoji.svg?branch=master)](https://travis-ci.org/dotanavi/dotamoji)

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

```
./target/release/build-dict [変換オプション] [出力ファイル] < [辞書テキスト]
```

### 確認

出力した辞書に全単語が載っているかをチェックします。

```
./target/release/test-dict array16 [出力された辞書ファイル] < [辞書テキスト]
```

### 形態素解析

標準入力を形態素解析します。

```
echo "すもももももももものうち" | ./target/release/analyze array16 [出力された辞書ファイル] [連結コストファイル]
```

## パフォーマンス

データ構造による辞書の構築時間の違いは以下のようになります。

| オプション | データ構造 | 文字コード | 構築時間 |
|--|--|--|--:|
| array8 | ダブル配列 | UTF-8 | 4m15.818s |
| array16 | ダブル配列 | UTF-16 | 1m19.848s |
| trie8 | トライ木 | UTF-8 | 0m0.696s |
| trie16 | トライ木 | UTF-16 | 0m0.457s |
| trie32 | トライ木 | UTF-32 | 0m0.450s |
| hash8 | 再帰HashMap | UTF-8 | 0m1.032s |
| hash16 | 再帰HashMap | UTF-16 | 0m0.549s |
| hash32 | 再帰HashMap | UTF-32 | 0m0.565s |
| fast8 | ダブル配列（空き要素をビットマップから検索） | UTF-8 | 0m4.189s |
| fast16 | ダブル配列（空き要素をビットマップから検索） | UTF-16 | 0m41.602s |
| trans8 | トライ木で構築し、ダブル配列に変換 | UTF-8 | 0m2.727s |
| trans16 | トライ木で構築し、ダブル配列に変換 | UTF-16 | 0m1.480s |
| trans32 | トライ木で構築し、ダブル配列に変換 | UTF-32 | 0m1.316s |
