use std::collections::HashMap;

extern crate common;

const SHAPE_A: [[char; 6]; 4] = [
    ['.', '.', '.', '.', '.', '.'], //
    ['.', '.', '.', '.', '.', '.'], //
    ['.', '.', '.', '.', '.', '.'], //
    ['.', '.', '#', '#', '#', '#'], //
];
const SHAPE_B: [[char; 6]; 4] = [
    ['.', '.', '.', '.', '.', '.'], //
    ['.', '.', '.', '#', '.', '.'], //
    ['.', '.', '#', '#', '#', '.'], //
    ['.', '.', '.', '#', '.', '.'], //
];
const SHAPE_C: [[char; 6]; 4] = [
    ['.', '.', '.', '.', '.', '.'], //
    ['.', '.', '.', '.', '#', '.'], //
    ['.', '.', '.', '.', '#', '.'], //
    ['.', '.', '#', '#', '#', '.'], //
];
const SHAPE_D: [[char; 6]; 4] = [
    ['.', '.', '#', '.', '.', '.'], //
    ['.', '.', '#', '.', '.', '.'], //
    ['.', '.', '#', '.', '.', '.'], //
    ['.', '.', '#', '.', '.', '.'], //
];
const SHAPE_E: [[char; 6]; 4] = [
    ['.', '.', '.', '.', '.', '.'], //
    ['.', '.', '.', '.', '.', '.'], //
    ['.', '.', '#', '#', '.', '.'], //
    ['.', '.', '#', '#', '.', '.'], //
];

fn solve_internal(input: &String, num_shapes: usize) -> usize {
    let mut level = vec![vec!['.'; 7]; 100000]; // level[y][x]
    let shapes = [SHAPE_A, SHAPE_B, SHAPE_C, SHAPE_D, SHAPE_E];

    let input_chars: Vec<char> = input.chars().collect();
    let mut top_y = 0;
    let mut input_idx = 0;
    let mut visited: HashMap<(usize, usize, String), (usize, usize)> = HashMap::new();
    let mut heights = vec![0; 10000];

    for shape_ctr in 0..num_shapes {
        let shape_idx = shape_ctr % 5;
        let shape = shapes[shape_idx];
        let mut curr_x = 0;
        let mut curr_y = top_y + 3;
        heights[shape_ctr] = top_y;

        // Use caching to check for repeating pattern
        if curr_y > 10 {
            let lines: String = (curr_y - 10..curr_y)
                .map(|y| level[y].iter().collect::<String>())
                .collect();

            let state = (shape_idx, input_idx, lines);
            let value = visited.entry(state).or_insert((0, 0));
            if *value == (0, 0) {
                *value = (top_y, shape_ctr);
            } else {
                let top_y_prev = value.0;
                let shape_ctr_prev = value.1;
                let diff_height = top_y - top_y_prev;
                let diff_shapes = shape_ctr - shape_ctr_prev;
                let num_steps = (num_shapes - shape_ctr) / diff_shapes;
                let num_left = num_shapes - shape_ctr - (num_steps * diff_shapes);
                let diff_height_rest = heights[shape_ctr_prev + num_left] - heights[shape_ctr_prev];
                return heights[shape_ctr_prev] + diff_height * (num_steps + 1) + diff_height_rest;
            }
        }

        loop {
            let dir = input_chars[input_idx];
            input_idx = (input_idx + 1) % input_chars.len();

            // try push sideways
            let dx: i32 = if dir == '<' { -1 } else { 1 };
            if can_move(shape, curr_x, curr_y as i32, dx, 0, &level) {
                curr_x += dx;
            }

            // try drop down
            if can_move(shape, curr_x, curr_y as i32, 0, -1, &level) {
                curr_y -= 1;
                continue;
            }

            // could not drop further, freeze and break out of loop
            for c in 0..6 {
                for r in 0..4 {
                    if shape[r][c] != '#' {
                        continue;
                    }
                    let (x, y) = (curr_x + c as i32, (3 - r) + curr_y);
                    top_y = std::cmp::max(top_y, y + 1);
                    level[y][x as usize] = '#';
                }
            }
            break;
        }
    }

    return top_y;
}

// go through all chars in shape and see if it collides
fn can_move(
    shape: [[char; 6]; 4],
    curr_x: i32,
    curr_y: i32,
    dx: i32,
    dy: i32,
    level: &Vec<Vec<char>>,
) -> bool {
    for c in 0..6 {
        for r in 0..4 {
            if shape[r][c] != '#' {
                continue;
            }
            let (x, y) = (curr_x + c as i32 + dx, (3 - r as i32) + curr_y + dy);
            if x < 0 || y < 0 || x > 6 || level[y as usize][x as usize] == '#' {
                return false;
            }
        }
    }

    return true;
}

fn part_one(input: &String) -> usize {
    solve_internal(input, 2022)
}

fn part_two(input: &String) -> usize {
    solve_internal(input, 1000000000000)
}

pub fn solve() {
    let input = common::read_file("2022/day17/input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();
        assert_eq!(part_one(&input), 3068);
        assert_eq!(part_two(&input), 1514285714288);
    }
}
