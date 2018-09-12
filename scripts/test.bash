#!/bin/bash

src_type="array16"
dst_type=
# dst_type="array16"
src="../dic/All.csv"
dst="tmp/tmp.bin"

set -Ceux

cargo build --release
time ./target/release/build-dict "$src_type" "$dst" < "$src"
./target/release/test-dict "${dst_type:-$src_type}" "$dst" < "$src"
