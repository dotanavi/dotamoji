#!/bin/bash

src_type="trans16"
dst_type="array16"
src="../dic/All.csv"
dst="tmp/tmp.bin"

set -Ceux

cargo build --release
./target/release/build-dict "$src_type" "$dst" < "$src"
./target/release/test-dict "${dst_type:-$src_type}" "$dst" < "$src"
