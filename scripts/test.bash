#!/bin/bash

type="array16"
src="../dic/Noun.csv"
dst="tmp/foo.bin"

set -Ceux

cargo build --release
time ./target/release/build-dict "$type" "$dst" < "$src"
./target/release/test-dict "$type" "$dst" < "$src"
