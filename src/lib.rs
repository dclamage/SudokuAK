pub mod board;

use board::Board;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn minlex(sudoku_str: &str) -> PyResult<String> {
    let sudoku = sudoku_str.chars().map(parse_digit).collect::<Vec<u8>>();
    if sudoku.len() != 81 {
        return Err(PyErr::new::<PyValueError, _>(
            "Invalid sudoku string. Must be exactly 81 characters long.",
        ));
    }

    let mut best_result: String = String::new();

    for rot in 0..8 {
        let mut cur_sudoku = match rot {
            0 => sudoku.clone(),
            1 => remap(&sudoku, |i, j| (i, 8 - j)),
            2 => remap(&sudoku, |i, j| (8 - i, j)),
            3 => remap(&sudoku, |i, j| (8 - i, 8 - j)),
            4 => remap(&sudoku, |i, j| (j, i)),
            5 => remap(&sudoku, |i, j| (j, 8 - i)),
            6 => remap(&sudoku, |i, j| (8 - j, i)),
            7 => remap(&sudoku, |i, j| (8 - j, 8 - i)),
            _ => sudoku.clone(),
        };

        renumber(&mut cur_sudoku);
        let cur_result = to_string(&cur_sudoku);
        if best_result.is_empty() || cur_result < best_result {
            best_result = cur_result;
        }
    }

    let best_result: String = best_result
        .chars()
        .map(|c| if c == '0' { '.' } else { c })
        .collect();
    Ok(best_result)
}

#[pyfunction]
fn solution_count(sudoku_str: &str, max_solutions: u64) -> u64 {
    let board = Board::from_given_str(sudoku_str);
    if board.is_none() {
        0
    } else {
        board.unwrap().count_solutions(max_solutions)
    }
}

#[pyfunction]
fn solve(sudoku_str: &str, random: bool) -> String {
    let board = Board::from_given_str(sudoku_str);
    if board.is_none() {
        String::new()
    } else {
        let board = board.unwrap();
        let solved = if random {
            board.solve_random()
        } else {
            board.solve()
        };

        if solved.is_none() {
            String::new()
        } else {
            solved.unwrap().to_string()
        }
    }
}

fn parse_digit(c: char) -> u8 {
    let c = c as u8;
    const ZERO: u8 = '0' as u8;
    const NINE: u8 = '9' as u8;
    if c >= ZERO && c <= NINE {
        c - ZERO
    } else {
        0
    }
}

fn to_string(sudoku: &Vec<u8>) -> String {
    let mut result = String::with_capacity(81);
    for i in 0..81 {
        result.push((sudoku[i] + '0' as u8) as char);
    }
    result
}

fn remap(sudoku: &Vec<u8>, rf: fn(usize, usize) -> (usize, usize)) -> Vec<u8> {
    let mut new_sudoku = Vec::with_capacity(81);
    for i in 0..9 {
        for j in 0..9 {
            let (i1, j1) = rf(i, j);
            new_sudoku.push(sudoku[i1 * 9 + j1]);
        }
    }
    new_sudoku
}

fn renumber(sudoku: &mut Vec<u8>) {
    let mut number_map = [0u8; 9];
    let mut next_number = 1;
    for i in 0..81 {
        let cur_val = sudoku[i] as usize;
        if cur_val != 0 {
            if number_map[cur_val - 1] == 0 {
                number_map[cur_val - 1] = next_number;
                next_number += 1;
            }
            sudoku[i] = number_map[cur_val - 1];
        }
    }
}

/// Implements the Python module pip, registers the class Engine
#[pymodule]
fn sudoku_ak(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(minlex, m)?)?;
    m.add_function(wrap_pyfunction!(solution_count, m)?)?;
    m.add_function(wrap_pyfunction!(solve, m)?)?;

    Ok(())
}
