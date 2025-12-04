fn parse_input(input: &String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        result.push(line.chars().collect());
    }
    return result;
}

fn count_neighs(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let r_min = row - std::cmp::min(row, 1);
    let r_max = std::cmp::min(row + 2, rows);

    let c_min = col - std::cmp::min(col, 1);
    let c_max = std::cmp::min(col + 2, cols);

    let mut result = 0;

    for row_i in r_min..r_max {
        for col_i in c_min..c_max {
            if row_i == row && col_i == col {
                continue;
            } else if grid[row_i][col_i] == '@' {
                result += 1;
            }
        }
    }

    return result;
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != '@' {
                continue;
            }
            let neighs = count_neighs(grid, row, col);
            if neighs < 4 {
                result += 1;
            }
        }
    }
    result
}

fn part_two(mut grid: Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut continue_removing = true;
    let mut total_removed = 0;
    let mut to_remove = Vec::new();
    to_remove.reserve(rows * cols);
    while continue_removing {
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != '@' {
                    continue;
                }
                let neighs = count_neighs(&grid, row, col);
                if neighs < 4 {
                    to_remove.push((row, col));
                }
            }
        }

        for (row, col) in &to_remove {
            grid[*row][*col] = '.';
        }

        continue_removing = !to_remove.is_empty();
        total_removed += to_remove.len();
        to_remove.clear();
    }

    total_removed
}

pub fn solve(filepath: &str) {
    let input = parse_input(
        &std::fs::read_to_string(filepath)
            .unwrap()
            .trim_end_matches('\n')
            .to_string(),
    );

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(input.clone()));
}
