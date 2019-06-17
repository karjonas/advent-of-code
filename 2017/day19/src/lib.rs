use std::fs::File;
use std::io::prelude::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve() {
    let mut file = File::open("2017/day19/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let grid: Vec<Vec<char>> = contents.split('\n').map(|v| v.chars().collect()).collect();

    let start_row = 0;
    let mut start_col = 0;

    for i in 0..grid[0].len() {
        if grid[0][i] == '|' {
            start_col = i;
            break;
        }
    }

    let mut stack = Vec::<(Direction, usize, usize, usize)>::new();
    stack.push((Direction::Down, start_row, start_col, 1));

    let mut max_steps = 0;
    let mut letters = String::new();

    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        let dir = curr.0;
        let row = curr.1;
        let col = curr.2;
        let steps = curr.3;

        if grid[row][col] == ' ' {
            continue;
        }

        let c = grid[row][col];

        if c != '|' && c != '-' && c != '+' && c != ' ' {
            letters.push(c);
            max_steps = steps;
        }

        if c != '+' {
            match dir {
                Direction::Up => {
                    stack.push((dir, row - 1, col, steps + 1));
                }
                Direction::Down => {
                    stack.push((dir, row + 1, col, steps + 1));
                }
                Direction::Left => {
                    stack.push((dir, row, col - 1, steps + 1));
                }
                Direction::Right => {
                    stack.push((dir, row, col + 1, steps + 1));
                }
            }
        } else {
            match dir {
                Direction::Up => {
                    stack.push((Direction::Left, row, col - 1, steps + 1));
                    stack.push((Direction::Right, row, col + 1, steps + 1));
                }
                Direction::Down => {
                    stack.push((Direction::Left, row, col - 1, steps + 1));
                    stack.push((Direction::Right, row, col + 1, steps + 1));
                }
                Direction::Left => {
                    stack.push((Direction::Up, row - 1, col, steps + 1));
                    stack.push((Direction::Down, row + 1, col, steps + 1));
                }
                Direction::Right => {
                    stack.push((Direction::Up, row - 1, col, steps + 1));
                    stack.push((Direction::Down, row + 1, col, steps + 1));
                }
            }
        }
    }

    println!("Part one: {}", letters);
    println!("Part two: {}", max_steps);
}
