#!/bin/bash
set -Ceu

cd "$(dirname $0)/.."

# -- configuration
out_dir=./target/release
matrix_file=../dic/matrix.def
save_dir=tmp
sentence="すもももももももものうち"

# -- functions
function analyze () {
  local build_type="$1"
  local binary_format="${2:-$1}"

  echo "----------------------------------------------------------------"
  echo
  echo "$sentence" | (set -x; "$out_dir/analyze" "$binary_format" "$save_dir/${build_type}_all.dic" "$matrix_file")
  if [ $? != 0 ]; then
    echo
    echo "コマンドが正常終了しませんでした。以下のコマンドでデバッグ実行してください"
    printf "echo \"%s\" | cargo run --bin analyze $binary_format $save_dir/${build_type}_all.dic ${matrix_file}\n" "$sentence"
    echo
  fi
}

# -- main
mkdir -p "$save_dir"
(
  set -x
  cargo build --release
)
echo

set +e # エラー無視
analyze array
analyze trie_a
analyze trie_b
analyze trie8
analyze trie16
analyze hash
analyze trans array
