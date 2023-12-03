use std::collections::HashMap;

#[derive(Clone)]
enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::UP => Direction::RIGHT,
        Direction::LEFT => Direction::UP,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::UP => Direction::LEFT,
        Direction::LEFT => Direction::DOWN,
        Direction::RIGHT => Direction::UP,
        Direction::DOWN => Direction::RIGHT,
    }
}

fn turn_around(dir: Direction) -> Direction {
    match dir {
        Direction::UP => Direction::DOWN,
        Direction::LEFT => Direction::RIGHT,
        Direction::RIGHT => Direction::LEFT,
        Direction::DOWN => Direction::UP,
    }
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let grid_in: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    assert!(grid_in.len() % 2 != 0);
    assert!(grid_in[0].len() % 2 != 0);

    let start_row = ((grid_in.len() - 1) / 2) as i32;
    let start_col = ((grid_in[0].len() - 1) / 2) as i32;

    let mut grid_start = HashMap::new();

    for row in 0..grid_in.len() {
        for col in 0..grid_in[row].len() {
            grid_start.insert((row as i32, col as i32), grid_in[row][col]);
        }
    }

    {
        let mut curr_row = start_row;
        let mut curr_col = start_col;

        let mut curr_dir = Direction::UP;

        let mut grid = grid_start.clone();

        let mut ctr = 0;
        for _ in 0..10000 {
            let val = grid.entry((curr_row, curr_col)).or_insert('.');
            if val.clone() == '#' {
                curr_dir = turn_right(curr_dir);
                *val = '.';
            } else {
                curr_dir = turn_left(curr_dir);
                *val = '#';
                ctr += 1;
            }
            match curr_dir.clone() {
                Direction::UP => {
                    curr_row -= 1;
                }
                Direction::LEFT => {
                    curr_col -= 1;
                }
                Direction::RIGHT => {
                    curr_col += 1;
                }
                Direction::DOWN => {
                    curr_row += 1;
                }
            }
        }

        println!("Part one: {}", ctr);
    }

    {
        let mut grid = grid_start.clone();
        let mut curr_row = start_row;
        let mut curr_col = start_col;
        let mut curr_dir = Direction::UP;
        let mut ctr = 0;
        for _ in 0..10000000 {
            let val = grid.entry((curr_row, curr_col)).or_insert('.');
            let val_prev = val.clone();
            if val_prev == '.' {
                curr_dir = turn_left(curr_dir);
                *val = 'W';
            } else if val_prev == 'W' {
                *val = '#';
                ctr += 1;
            } else if val_prev == '#' {
                curr_dir = turn_right(curr_dir);
                *val = 'F';
            } else if val_prev == 'F' {
                curr_dir = turn_around(curr_dir);
                *val = '.';
            }

            match curr_dir.clone() {
                Direction::UP => {
                    curr_row -= 1;
                }
                Direction::LEFT => {
                    curr_col -= 1;
                }
                Direction::RIGHT => {
                    curr_col += 1;
                }
                Direction::DOWN => {
                    curr_row += 1;
                }
            }
        }

        println!("Part two: {:?}", ctr);
    }
}
