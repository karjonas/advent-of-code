fn parse_input(input: &String) -> Vec<Vec<char>> {
    return input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    let mut repeat = true;
    let width = grid[0].len();
    let height = grid.len();
    while repeat {
        repeat = false;
        for y_i in 0..height - 1 {
            let y = height - y_i - 1;
            for x in 0..width {
                if grid[y][x] == 'O' && grid[y - 1][x] == '.' {
                    grid[y - 1][x] = 'O';
                    grid[y][x] = '.';
                    repeat = true;
                }
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    let mut repeat = true;
    let width = grid[0].len();
    while repeat {
        repeat = false;
        for y in 0..grid.len() {
            for x_i in 0..width - 1 {
                let x = width - x_i - 1;
                if grid[y][x] == 'O' && grid[y][x - 1] == '.' {
                    grid[y][x - 1] = 'O';
                    grid[y][x] = '.';
                    repeat = true;
                }
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let mut repeat = true;
    let width = grid[0].len();

    while repeat {
        repeat = false;
        for y in 0..grid.len() - 1 {
            for x in 0..width {
                if grid[y][x] == 'O' && grid[y + 1][x] == '.' {
                    grid[y + 1][x] = 'O';
                    grid[y][x] = '.';
                    repeat = true;
                }
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    let mut repeat = true;
    let width = grid[0].len();

    while repeat {
        repeat = false;
        for y in 0..grid.len() {
            for x in 0..width - 1 {
                if grid[y][x] == 'O' && grid[y][x + 1] == '.' {
                    grid[y][x + 1] = 'O';
                    grid[y][x] = '.';
                    repeat = true;
                }
            }
        }
    }
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let mut grid_new = grid.clone();
    tilt_north(&mut grid_new);

    let mut sum = 0;
    for y in 0..grid_new.len() {
        let value = grid_new.len() - y;
        let num_rocks = grid_new[y].iter().filter(|v| **v == 'O').count();
        sum += value * num_rocks;
    }

    return sum;
}

fn part_two(grid: &Vec<Vec<char>>) -> usize {
    let mut grid_new = grid.clone();
    let mut grids: Vec<Vec<Vec<char>>> = Vec::new();
    let mut idx = 0;
    const NUM_REPEATS: usize = 1000000000;

    while idx < NUM_REPEATS {
        tilt_north(&mut grid_new);
        tilt_west(&mut grid_new);
        tilt_south(&mut grid_new);
        tilt_east(&mut grid_new);

        let mut repeat_idx = 0;
        let mut found = false;
        for grid_i in &grids {
            if *grid_i == grid_new {
                found = true;
                break;
            }
            repeat_idx += 1;
        }

        if found {
            let cycle = idx - repeat_idx;
            let steps = (NUM_REPEATS - idx - 1) % cycle;
            idx = repeat_idx + steps;
            grid_new = grids[idx].clone();
            break;
        }

        idx += 1;
        grids.push(grid_new.clone());
    }

    let mut sum = 0;
    for y in 0..grid_new.len() {
        let value = grid_new.len() - y;
        let num_rocks = grid_new[y].iter().filter(|v| **v == 'O').count();
        sum += value * num_rocks;
    }

    return sum;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let grid = parse_input(&input);

    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}
