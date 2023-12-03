extern crate common;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct State {
    map: Vec<Vec<char>>, // map[row][col]
    movements: Vec<String>,
}

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;
const NUM_DIRS: usize = 4;

fn parse(input: &String) -> State {
    let mut map: Vec<Vec<char>> = Vec::new(); // map[row][col]
    let lines: Vec<_> = input.lines().collect();

    let mut max_cols = 0;
    for idx in 0..lines.len() - 2 {
        map.push(lines[idx].chars().collect());
        max_cols = std::cmp::max(max_cols, map[idx].len());
    }

    // fill empty space with spaces
    for i in 0..map.len() {
        map[i].resize(max_cols, ' ');
    }

    let movements: Vec<_> = lines
        .last()
        .unwrap()
        .replace("L", " L ")
        .replace("R", " R ")
        .split_ascii_whitespace()
        .map(|v| v.to_string())
        .collect();

    return State {
        map: map,
        movements: movements,
    };
}

fn is_outside(pos: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    let num_rows = map.len() as i32;
    let num_cols = map[0].len() as i32;
    let outside = pos.0 < 0
        || pos.0 >= num_rows
        || pos.1 < 0
        || pos.1 >= num_cols
        || map[pos.0 as usize][pos.1 as usize] == ' ';
    outside
}

fn walk_flat(dir: usize, pos: (i32, i32), map: &Vec<Vec<char>>) -> (i32, i32) {
    let (dy, dx) = match dir {
        UP => (-1, 0),
        RIGHT => (0, 1),
        DOWN => (1, 0),
        LEFT => (0, -1),
        _ => (0, 0),
    };

    let num_rows = map.len() as i32;
    let num_cols = map[0].len() as i32;

    let mut pos_next = (
        (num_rows + pos.0 + dy) % num_rows,
        (num_cols + pos.1 + dx) % num_cols,
    );

    // Step until hitting floor or wall
    while map[pos_next.0 as usize][pos_next.1 as usize] == ' ' {
        pos_next = (
            (num_rows + pos_next.0 + dy) % num_rows,
            (num_cols + pos_next.1 + dx) % num_cols,
        );
    }

    // If open space we move
    if map[pos_next.0 as usize][pos_next.1 as usize] == '.' {
        return pos_next;
    }

    // otherwise return same pos
    return pos;
}

// credit AxlLind: https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/22.rs
fn wrap_cube(dir: usize, pos: (i32, i32)) -> ((i32, i32), usize) {
    let y = pos.0;
    let x = pos.1;
    let (qr, qc, dir_next) = match (y / 50, x / 50, dir) {
        (0, 1, UP) => (3, 0, RIGHT),
        (0, 1, LEFT) => (2, 0, RIGHT),
        (0, 2, UP) => (3, 0, UP),
        (0, 2, RIGHT) => (2, 1, LEFT),
        (0, 2, DOWN) => (1, 1, LEFT),
        (1, 1, RIGHT) => (0, 2, UP),
        (1, 1, LEFT) => (2, 0, DOWN),
        (2, 0, UP) => (1, 1, RIGHT),
        (2, 0, LEFT) => (0, 1, RIGHT),
        (2, 1, RIGHT) => (0, 2, LEFT),
        (2, 1, DOWN) => (3, 0, LEFT),
        (3, 0, RIGHT) => (2, 1, UP),
        (3, 0, DOWN) => (0, 2, DOWN),
        (3, 0, LEFT) => (0, 1, DOWN),
        _ => unreachable!(),
    };
    let (dr, dc) = (y % 50, x % 50);
    let i = [dr, 49 - dc, 49 - dr, dc][dir];
    let (nr, nc) = [(i, 0), (0, 49 - i), (49 - i, 49), (49, i)][dir_next];
    let pos_next = (qr * 50 + nr, qc * 50 + nc);
    return (pos_next, dir_next);
}

fn walk_cube(dir: usize, pos: (i32, i32), map: &Vec<Vec<char>>) -> ((i32, i32), usize) {
    let (dy, dx) = match dir {
        UP => (-1, 0),
        RIGHT => (0, 1),
        DOWN => (1, 0),
        LEFT => (0, -1),
        _ => (0, 0),
    };

    let mut dir_next = dir;
    let mut pos_next = (pos.0 + dy, pos.1 + dx);
    if is_outside(pos_next, map) {
        (pos_next, dir_next) = wrap_cube(dir, pos);
    }
    if !is_outside(pos_next, map) && map[pos_next.0 as usize][pos_next.1 as usize] == '.' {
        return (pos_next, dir_next);
    }

    // otherwise return same pos
    return (pos, dir);
}

fn part_one(input: &String) -> i32 {
    let state = parse(input);
    let mut pos = (0, 0);
    let mut dir = RIGHT;
    for action in state.movements {
        let number = action.parse().unwrap_or(0);
        for _ in 0..number {
            pos = walk_flat(dir, pos, &state.map);
        }

        if action == "L" {
            dir = (dir + NUM_DIRS - 1) % NUM_DIRS;
        } else if action == "R" {
            dir = (dir + 1) % NUM_DIRS;
        }
    }

    let row = (pos.0 + 1) * 1000;
    let column = (pos.1 + 1) * 4;
    let facing = dir as i32;

    return row + column + facing;
}

fn part_two(input: &String) -> i32 {
    let state = parse(input);
    let mut pos = (0, 50);
    let mut dir = RIGHT;
    for action in state.movements {
        let number = action.parse().unwrap_or(0);
        for _ in 0..number {
            (pos, dir) = walk_cube(dir, pos, &state.map);
        }

        if action == "L" {
            dir = (dir + NUM_DIRS - 1) % NUM_DIRS;
        } else if action == "R" {
            dir = (dir + 1) % NUM_DIRS;
        }
    }

    let row = (pos.0 + 1) * 1000;
    let column = (pos.1 + 1) * 4;
    let facing = dir as i32;

    return row + column + facing;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
            .to_string();
        assert_eq!(part_one(&input), 6032);
    }
}
