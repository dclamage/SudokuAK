#!/bin/bash

cargo build --release
mkdir -p ./package
cp ./target/release/libsudoku_ak.so ./package/sudoku_ak.so
