use std::collections::HashSet;

extern crate common;

const EMPTY: u8 = 0;
const UP: u8 = 1 << 1;
const DOWN: u8 = 1 << 2;
const LEFT: u8 = 1 << 3;
const RIGHT: u8 = 1 << 4;
const WALL: u8 = 1 << 5;

fn parse(input: &String) -> Vec<Vec<Vec<u8>>> {
    let mut maps = Vec::new();
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let v = match c {
                '.' => EMPTY,
                '^' => UP,
                'v' => DOWN,
                '<' => LEFT,
                '>' => RIGHT,
                '#' => WALL,
                _ => 0,
            };
            row.push(v);
        }
        map.push(row);
    }

    let rows = map.len();
    let cols = map[0].len();
    let mut map_empty = map.clone();
    {
        // make empty map
        for y in 1..rows - 1 {
            for x in 1..cols - 1 {
                map_empty[y][x] = EMPTY;
            }
        }
    }

    maps.push(map.clone());
    let mut map_prev = map.clone();

    loop {
        let mut map = map_empty.clone();
        for y in 1..rows - 1 {
            for x in 1..cols - 1 {
                let v = map_prev[y][x];

                let is_up = UP & v != 0;
                let is_down = DOWN & v != 0;
                let is_left = LEFT & v != 0;
                let is_right = RIGHT & v != 0;

                if is_down {
                    let y_next = if y == rows - 2 { 1 } else { y + 1 };
                    map[y_next][x] |= DOWN;
                }
                if is_right {
                    let x_next = if x == cols - 2 { 1 } else { x + 1 };
                    map[y][x_next] |= RIGHT;
                }
                if is_up {
                    let y_next = if y == 1 { rows - 2 } else { y - 1 };
                    map[y_next][x] |= UP;
                }
                if is_left {
                    let x_next = if x == 1 { cols - 2 } else { x - 1 };
                    map[y][x_next] |= LEFT;
                }
            }
        }

        if maps.contains(&map) {
            break;
        }
        maps.push(map.clone());
        map_prev = map;
    }

    return maps;
}

fn part_both(input: &String, part_one: bool) -> usize {
    let maps = parse(input);
    let rows = maps[0].len();
    let cols = maps[0][0].len();
    let goal_y = rows - 1;
    let goal_x = cols - 2;
    let num_maps = maps.len();

    let mut states = Vec::new();
    states.push(((1, 0), 0, 0)); // ((x,y), minute, trip (0 goal, 1 start, 2 goal))

    let mut visited = HashSet::new();

    while !states.is_empty() {
        let ((x, y), minute, mut trip) = states.pop().unwrap();
        let state = ((x, y), minute % num_maps, trip);

        if (trip == 2 || part_one) && (x, y) == (goal_x, goal_y) {
            return minute;
        }

        if visited.contains(&state) {
            continue;
        }

        if trip == 0 && (x, y) == (goal_x, goal_y) {
            trip = 1;
        } else if trip == 1 && (x, y) == (1, 0) {
            trip = 2;
        }

        visited.insert(state);

        // get next minute map
        let map = &maps[(minute + 1) % num_maps];

        // try move all directions
        if map[y][x] == EMPTY {
            states.push(((x, y), minute + 1, trip)); // stay
        }
        if y < rows - 1 && map[y + 1][x] == EMPTY {
            states.push(((x, y + 1), minute + 1, trip)); // down
        }
        if y > 0 && map[y - 1][x] == EMPTY {
            states.push(((x, y - 1), minute + 1, trip)); // up
        }
        if x < cols - 1 && map[y][x + 1] == EMPTY {
            states.push(((x + 1, y), minute + 1, trip)); // right
        }
        if x > 0 && map[y][x - 1] == EMPTY {
            states.push(((x - 1, y), minute + 1, trip)); // left
        }

        // sort by minute
        states.sort_by(|a, b| b.1.cmp(&a.1));
    }

    0
}

fn part_one(input: &String) -> usize {
    part_both(input, true)
}

fn part_two(input: &String) -> usize {
    part_both(input, false)
}

pub fn solve() {
    let input = common::read_file("2022/day24/input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
            .to_string();
        assert_eq!(part_one(&input), 18);
        assert_eq!(part_two(&input), 54);
    }
}
