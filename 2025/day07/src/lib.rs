fn parse_input(input: &String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    return grid;
}

fn part_one_two(mut grid: Vec<Vec<char>>) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut num_splits = 0;
    let mut grid_splits: Vec<Vec<usize>> = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            let in_grid = |d_r, d_c| {
                let r_n = r as i64 + d_r;
                let c_n = c as i64 + d_c;
                r_n >= 0 && r_n < rows as i64 && c_n >= 0 && c_n < cols as i64
            };

            let get_pos = |grid: &Vec<Vec<char>>, d_r, d_c| {
                if in_grid(d_r, d_c) {
                    return grid[(r as i64 + d_r) as usize][(c as i64 + d_c) as usize];
                }
                return '.';
            };

            if grid[r][c] == 'S' && get_pos(&grid, 1, 0) == '.' {
                grid[r + 1][c] = '|';
                grid_splits[r + 1][c] = 1;
            }

            if grid[r][c] == '^' && get_pos(&grid, -1, 0) == '|' {
                num_splits += 1;
                let splits_above = grid_splits[r - 1][c];
                let left = get_pos(&grid, 0, -1);
                let right = get_pos(&grid, 0, 1);
                if left == '.' || left == '|' {
                    grid[r][c - 1] = '|';
                    grid_splits[r][c - 1] += splits_above;
                }
                if right == '.' || right == '|' {
                    grid[r][c + 1] = '|';
                    grid_splits[r][c + 1] += splits_above;
                }
            }

            if grid[r][c] != '^' && get_pos(&grid, -1, 0) == '|' {
                grid[r][c] = '|';
                grid_splits[r][c] += grid_splits[r - 1][c];
            }
        }
    }

    let mut num_multi = 0;
    for c in 0..cols {
        num_multi += grid_splits[rows - 1][c];
    }

    (num_splits, num_multi)
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let grid = parse_input(&input);
    let (p1, p2) = part_one_two(grid);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}
