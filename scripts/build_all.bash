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
  local dic_path="$save_dir/${build_type}_all.dic"

  echo
  echo "----------------------------------------------------------------"
  echo
  (
    set -x
    time "$out_dir/build-dict" "$build_type" "$dic_path.tmp" < $dic_file
    "$out_dir/test-dict" "$binary_format" "$dic_path.tmp" < $dic_file
    mv "$dic_path.tmp" "$dic_path"
  )
}

# -- main
mkdir -p "$save_dir"
(
  set -x
  cargo build --release
)

build trans8 array8
build trans16 array16
build trie8
build trie16
build trie32
build hash8
build hash16
build hash32
build array8
build array16
