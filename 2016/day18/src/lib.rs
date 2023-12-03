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

fn solve_internal(input: &String, rows: usize) -> usize {
    let start: Vec<bool> = input
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

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part 1: {}", solve_internal(&input, 40));
    println!("Part 2: {}", solve_internal(&input, 400000));
}
