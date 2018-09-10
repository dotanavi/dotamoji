#!/bin/bash
set -Ceu

cd "$(dirname $0)/.."

# -- configuration
out_dir=./target/release
dic_file=../dic/All.csv

# -- functions
function measure () {
  local type="$1"
  local enc="$2"
  local desc="$3"
  local time=$( (time $out_dir/build-dict $type - < $dic_file > /dev/null) 2>&1 | grep real | cut -f 2 )
  echo "| $type | $enc | $desc | $time |"
}

cargo build --release --quiet

echo "| オプション | 文字コード | データ構造 | 構築時間 |"
echo "|--|--|--|--:|"

measure array8  "UTF-8"          "ダブル配列"
measure array16 "UTF-16"         "ダブル配列"

measure trie8  "UTF-8"           "トライ木"
measure trie16 "UTF-16"          "トライ木"
measure trie32 "ユニコードポイント" "トライ木"

measure hash8  "UTF-8"           "再帰HashMap"
measure hash16 "UTF-16"          "再帰HashMap"
measure hash32 "ユニコードポイント" "再帰HashMap"

measure fast8  "UTF-8"            "ビットマップキャッシュを用いてダブル配列を構築"
measure fast16 "UTF-16"           "ビットマップキャッシュを用いてダブル配列を構築"

measure trans8  "UTF-8"           "トライ木を構築し、ダブル配列に変換"
measure trans16 "UTF-16"          "トライ木を構築し、ダブル配列に変換"
measure trans32 "ユニコードポイント" "トライ木を構築し、ダブル配列に変換"
