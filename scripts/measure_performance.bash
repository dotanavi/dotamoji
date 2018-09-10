#!/bin/bash
set -Ceu

cd "$(dirname $0)/.."

# -- configuration
out_dir=./target/release
dic_file=../dic/All.csv

# -- names
declare -A encoding;
encoding=(
  ["8"]="UTF-8"
  ["16"]="UTF-16"
  ["32"]="UTF-32"
)

declare -A structure;
structure=(
  ["array"]="ダブル配列"
  ["trie"]="トライ木"
  ["hash"]="再帰HashMap"
  ["fast"]="ダブル配列（空き要素をビットマップから検索）"
  ["trans"]="トライ木で構築し、ダブル配列に変換"
)


# -- functions
function measure () {
  local struct="${structure[$1]}"
  local enc="${encoding[$2]}"
  local type="${1}${2}"
  local time=$( (time $out_dir/build-dict $type - < $dic_file > /dev/null) 2>&1 | grep real | cut -f 2 )
  echo "| $type | $struct | $enc | $time |"
}

# -- main
cargo build --release --quiet

echo "| オプション | データ構造 | 文字コード | 構築時間 |"
echo "|--|--|--|--:|"

measure array 8
measure array 16

measure trie 8
measure trie 16
measure trie 32

measure hash 8
measure hash 16
measure hash 32

measure fast 8
measure fast 16

measure trans 8
measure trans 16
measure trans 32
