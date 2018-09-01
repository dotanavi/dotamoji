#!/bin/bash
set -Ceu

cd "$(dirname $0)/.."

# -- configuration
out_dir=./target/release
dic_file=../dic/All.csv
save_dir=tmp

# -- functions
function build () {
  local build_type="$1"
  local binary_format="${2:-$1}"

  echo
  (
    set -x
    time "$out_dir/build-dict" "$build_type" "$save_dir/${build_type}_all.dic" < $dic_file
    time "$out_dir/test-dict" "$binary_format" "$save_dir/${build_type}_all.dic" < $dic_file
  )
}

# -- main
mkdir -p "$save_dir"
(
  set -x
  cargo build --release
)

build trans array
build trie
build hash
build array
