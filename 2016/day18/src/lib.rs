use std::fs::File;
use std::io::prelude::*;

fn is_trap(r: usize, c: usize, num_cols: usize, grid: &Vec<Vec<bool>>) -> bool {
    let left = if c == 0 { false } else { grid[r - 1][c - 1] };
    let center = grid[r - 1][c];
    let right = if c == num_cols - 1 {
        false
    } else {
        grid[r - 1][c + 1]
    };
    let p = (left, center, right);

    return p == (true, true, false)
        || p == (false, true, true)
        || p == (true, false, false)
        || p == (false, false, true);
}

fn generate_rows(start: Vec<bool>, num_rows_inclusive: usize) -> Vec<Vec<bool>> {
    let num_cols = start.len();
    let mut output: Vec<Vec<bool>> = Vec::new();
    output.resize(num_rows_inclusive, Vec::new());
    output[0] = start;

    for row in 1..(num_rows_inclusive) {
        output[row].resize(num_cols, false);
        for col in 0..num_cols {
            output[row][col] = is_trap(row, col, num_cols, &output);
        }
    }

    return output;
}

fn solve_internal(path: &str, rows: usize) -> usize {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let start: Vec<bool> = contents
        .chars()
        .filter(|&c| c == '.' || c == '^')
        .map(|c| c == '^')
        .collect();
    let output = generate_rows(start, rows);
    let nums = output.iter().fold(0, |sum, v| {
        sum + v.iter().fold(0, |sum, &v0| sum + if !v0 { 1 } else { 0 })
    });
    return nums;
}

pub fn solve() {
    println!("Part 1: {}", solve_internal("2016/day18/input", 40));
    println!("Part 2: {}", solve_internal("2016/day18/input", 400000));
}
