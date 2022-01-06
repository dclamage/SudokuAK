# Anti-Knight Sudoku Solver

*Created by David Clamage (Rangsk)*

This Rust library is intended to hook into Python 3 and allow for quickly solving Anti-Knight Sudoku puzzles. It also has functionality for putting sudoku puzzles into minimal lexicographical form (minlex).

## Building and Deploying

I have provided scripts for Windows, OSX, and Linux.

Prerequisites:
 - Ensure you have Python 3 installed.
 - [Install rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) (`rustc` and `cargo`)
 - [Install git](https://www.atlassian.com/git/tutorials/install-git)

### Windows: 

```cmd
git clone https://github.com/dclamage/SudokuAK.git
cd SudokuAK
build.bat
```

Copy `package\sudoku_ak.pyd` to the same folder as your python project.

### Linux:

```sh
git clone https://github.com/dclamage/SudokuAK.git
cd SudokuAK
./build-linux.sh
```

Copy `package\sudoku_ak.so` to the same folder as your python project.


### MacOS:

```sh
git clone https://github.com/dclamage/SudokuAK.git
cd SudokuAK
./build-osx.sh
```


Copy `package\sudoku_ak.so` to the same folder as your python project.

## Usage

See [test.py](package/test.py) for an example script.

Basics:

```py
# This import will work as long as sudoku_ak.pyd (Windows) / sudoku_ak.so (OSX/Linux) are in the same folder as the script.
import sudoku_ak

# The sudoku string must be exactly 81 characters long. Any non-numerical digit is treated as a non-given.
sudoku_string = '..................................1.......2.34....5....6...7..........8..........'

# Get the exact number of solutions to the puzzle. The second parameter is a maximum number of solutions to return, or 0 for no limit.
count = sudoku_ak.solution_count(sudoku_string, 0)

# Get a solution to the puzzle. The second parameter is whether the solution should be random (different every time).
# When non-random, the solution is not guaranteed to be any specific solution, but it will be consistent every time
# it is called on the same input.
solved = sudoku_ak.solve(sudoku_string, False)

# The minlexed output will be a string with '.' for non-givens
minlexed = sudoku_ak.minlex('9.......1......7....8......7.....2...4..68..........5..6..7..............3.....7.')

# minlexed now contains: '........12..3..........4.2.....5......2.6...................5..7.6.8.........2..9'

```
