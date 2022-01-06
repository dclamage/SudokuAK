#!/bin/bash

cargo build --release
mkdir -p ./package
cp ./target/release/libsudoku_ak.dylib ./package/sudoku_ak.so
