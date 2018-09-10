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
analyze array8
analyze array16
analyze trie8
analyze trie16
analyze trie32
analyze hash8
analyze hash16
analyze hash32
analyze fast8 array8
analyze fast16 array16
analyze trans8 array8
analyze trans16 array16
analyze trans32 array32
