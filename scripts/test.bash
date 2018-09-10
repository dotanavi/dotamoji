#!/bin/bash

src_type="fast16"
dst_type="array16"
src="../dic/Noun.csv"
dst="tmp/foo.bin"

set -Ceux

cargo build --release
time ./target/release/build-dict "$src_type" "$dst" < "$src"
./target/release/test-dict "${dst_type:-$src_type}" "$dst" < "$src"
