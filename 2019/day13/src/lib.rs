extern crate common;
extern crate intcode;

use std::collections::HashMap;
use std::collections::VecDeque;

const TILE_EMPTY: i64 = 0;
const TILE_WALL: i64 = 1;
const TILE_BLOCK: i64 = 2;
const TILE_HORIZONTAL: i64 = 3;
const TILE_BALL: i64 = 4;

fn solve_part_one(memory: Vec<i64>) -> usize {
    let (_, output_numbers, _, _, _) = intcode::run(memory.clone(), VecDeque::new(), 0, 0);
    let map = memory_to_tiles(output_numbers);
    let mut num_b = 0;
    for (_, v) in map {
        num_b += if v == TILE_BLOCK { 1 } else { 0 };
    }
    return num_b;
}

fn memory_to_tiles(numbers: Vec<i64>) -> HashMap<(i64, i64), i64> {
    let mut tiles = HashMap::new();

    let stride = 3;
    let mut i = 2;
    while i < numbers.len() {
        let x = numbers[i - 2];
        let y = numbers[i - 1];
        let tile = numbers[i];
        tiles.insert((x, y), tile);
        i += stride;
    }
    return tiles;
}

fn get_tile_pos(tiles: &HashMap<(i64, i64), i64>, tile: i64) -> i64 {
    for (k, v) in tiles {
        if v.clone() == tile {
            return k.0;
        }
    }
    panic!("Tile not found");
}

fn print_board(numbers: Vec<i64>) {
    let board = memory_to_tiles(numbers);
    let mut min = (0, 0);
    let mut max = (0, 0);
    let mut score = 0;

    for (k, v) in board.clone() {
        if k == (-1, 0) {
            score = v;
            continue;
        }
        min.0 = std::cmp::min(min.0, k.0);
        min.1 = std::cmp::min(min.1, k.1);

        max.0 = std::cmp::max(max.0, k.0);
        max.1 = std::cmp::max(max.1, k.1);
    }

    let mut s = String::new();

    for y in min.1..(max.1 + 1) {
        s.push('\n');
        for x in min.0..(max.0 + 1) {
            let tile = board.get(&(x, y)).unwrap_or(&TILE_EMPTY).clone();
            let c = if tile == TILE_EMPTY {
                ' '
            } else if tile == TILE_BALL {
                'O'
            } else if tile == TILE_BLOCK {
                'X'
            } else if tile == TILE_HORIZONTAL {
                'H'
            } else if tile == TILE_WALL {
                'W'
            } else {
                '-'
            };
            s.push(c);
        }
    }
    println!("{}", s);
    println!("score {}", score);
}

fn solve_part_two(mut memory: Vec<i64>) -> usize {
    let debug = false;
    memory[0] = 2;
    let mut next_dir = 0;
    let mut score;
    loop {
        let input = VecDeque::from([next_dir].to_vec());
        let (memory_new, output_numbers, _, _, halted) =
            intcode::run(memory.clone(), input.clone(), 0, 0);
        memory = memory_new;

        if debug {
            print_board(output_numbers.clone());
        }
        let tiles = memory_to_tiles(output_numbers);
        let ball_x = get_tile_pos(&tiles, TILE_BALL);
        let horiz_x = get_tile_pos(&tiles, TILE_HORIZONTAL);
        score = tiles.get(&(-1, 0)).unwrap().clone();
        next_dir = if ball_x < horiz_x {
            -1
        } else if ball_x > horiz_x {
            1
        } else {
            0
        };
        if halted {
            break;
        }
    }
    return score as usize;
}

pub fn solve() {
    let input = common::read_file("2019/day13/input");
    let memory = intcode::parse_input(input.as_str());

    println!("Part one: {}", solve_part_one(memory.clone()));
    println!("Part two: {}", solve_part_two(memory));
}
