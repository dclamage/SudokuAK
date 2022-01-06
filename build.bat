@echo off
cargo build --release
if not exist ".\package" mkdir ".\package"
copy ".\target\release\sudoku_ak.dll" ".\package\sudoku_ak.pyd"
