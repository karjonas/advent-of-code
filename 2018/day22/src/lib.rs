extern crate common;

use std::collections::VecDeque;

const INPUT: (usize, usize, usize) = (3198, 12, 757);

const TYPE_ROCKY: char = '.';
const TYPE_NARROW: char = '|';
const TYPE_WET: char = '=';

fn idx_to_type(idx: usize) -> char {
    if idx == 0 {
        return TYPE_ROCKY;
    } else if idx == 1 {
        return TYPE_WET;
    } else if idx == 2 {
        return TYPE_NARROW;
    }
    assert!(false);
    return '-';
}

fn type_to_risk(t: char) -> usize {
    if t == TYPE_ROCKY {
        return 0;
    }
    if t == TYPE_WET {
        return 1;
    }
    if t == TYPE_NARROW {
        return 2;
    }
    assert!(false);
    return 0;
}

fn generate_map(width: usize, height: usize, depth: usize) -> Vec<Vec<char>> {
    let mut grid = common::filled_vector(height, common::filled_vector(width, ' '));
    let mut erosion_level = common::filled_vector(height, common::filled_vector(width, 0));
    let mut geologic_index = common::filled_vector(height, common::filled_vector(width, 0));

    for y in 0..height {
        geologic_index[y][0] = y * 48271;
        erosion_level[y][0] = (geologic_index[y][0] + depth) % 20183;
    }
    for x in 0..width {
        geologic_index[0][x] = x * 16807;
        erosion_level[0][x] = (geologic_index[0][x] + depth) % 20183;
    }

    for y in 1..height {
        for x in 1..width {
            geologic_index[y][x] = erosion_level[y][x - 1] * erosion_level[y - 1][x];
            if (x, y) == (INPUT.1, INPUT.2) {
                geologic_index[y][x] = 0; // Horrible special case :(
            }
            erosion_level[y][x] = (geologic_index[y][x] + depth) % 20183;
        }
    }

    for y in 0..height {
        for x in 0..width {
            let v = erosion_level[y][x] % 3;
            grid[y][x] = idx_to_type(v);
        }
    }

    return grid;
}

fn calculate_risk(grid: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for line in grid.iter() {
        for v in line.iter() {
            result += type_to_risk(*v);
        }
    }

    // Remove goal
    result -= type_to_risk(grid[grid.len() - 1][grid[0].len() - 1]);

    return result;
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        let s = line.iter().collect::<String>();
        println!("{}", s);
    }
}

fn get_adjacent(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    res.reserve(4);

    if y > 0 {
        res.push((x, y - 1));
    }
    if x > 0 {
        res.push((x - 1, y));
    }
    if x + 1 < width {
        res.push((x + 1, y));
    }
    if y + 1 < height {
        res.push((x, y + 1));
    }
    return res;
}

fn valid_tool(c: char, t: usize) -> bool {
    if c == TYPE_ROCKY && t == 0 {
        return false;
    }
    if c == TYPE_WET && t == 1 {
        return false;
    }
    if c == TYPE_NARROW && t == 2 {
        return false;
    }
    return true;
}

fn build_distances(grid: &Vec<Vec<char>>) -> Vec<Vec<[usize; 3]>> {
    let width = grid[0].len();
    let height = grid.len();

    let mut dist = common::filled_vector(
        height,
        common::filled_vector(width, [std::usize::MAX, std::usize::MAX, std::usize::MAX]),
    );

    let mut stack: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();

    // initial state
    stack.push_back((0, 0, 0, 1));

    while !stack.is_empty() {
        let (x, y, d, t) = stack.pop_front().unwrap();

        if dist[y][x][t] <= d || !valid_tool(grid[y][x], t) {
            continue;
        }
        dist[y][x][t] = d;

        let adjacents = get_adjacent(x, y, width, height);

        for i in 0..3 {
            if t == i || !valid_tool(grid[y][x], i) {
                continue;
            }
            stack.push_back((x, y, d + 7, i));
        }

        for (x_adj, y_adj) in adjacents {
            stack.push_back((x_adj, y_adj, d + 1, t));
        }
    }

    return dist;
}

fn part_one() {
    let m = generate_map(INPUT.1 + 1, INPUT.2 + 1, INPUT.0);
    if false {
        print_grid(&m);
    }
    println!("Part one: {}", calculate_risk(&m));
}

fn part_two() {
    let m = generate_map(
        std::cmp::max(INPUT.1, INPUT.2) + 300 + 1,
        std::cmp::max(INPUT.1, INPUT.2) + 300 + 1,
        INPUT.0,
    );
    if false {
        print_grid(&m);
    }
    let dists = build_distances(&m);

    println!("Part two: {}", dists[INPUT.2][INPUT.1][1]);
}

pub fn solve(_filepath: &str) {
    part_one();
    part_two();
}
