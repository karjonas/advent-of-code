extern crate common;

use std::collections::HashMap;
use std::collections::HashSet;

type Grid = [[bool; 5]; 5];
type MultiGrid = HashMap<i32, Grid>;

fn empty_grid() -> Grid {
    return [[false; 5]; 5];
}

fn parse_grid(input: String) -> Grid {
    let mut grid = empty_grid();
    let mut y = 0;

    for line in input.lines() {
        let mut x = 0;
        for c in line.trim().chars() {
            grid[y][x] = c == '#';
            x += 1;
        }
        y += 1;
    }
    return grid;
}

fn step(input: &Grid) -> Grid {
    let mut output = input.clone();

    for y in 0..5 {
        for x in 0..5 {
            let left = if x > 0 { input[y][x - 1] } else { false };
            let right = if x < 4 { input[y][x + 1] } else { false };
            let up = if y > 0 { input[y - 1][x] } else { false };
            let down = if y < 4 { input[y + 1][x] } else { false };

            let is_bug = input[y][x];
            let num_neigh_bugs = left as u8 + right as u8 + up as u8 + down as u8;

            if is_bug && num_neigh_bugs != 1 {
                output[y][x] = false;
            } else if !is_bug && (num_neigh_bugs == 1 || num_neigh_bugs == 2) {
                output[y][x] = true;
            }
        }
    }
    return output;
}

fn first_repeating(input: &Grid) -> Grid {
    let mut visited = HashSet::new();

    let mut curr_grid = input.clone();
    visited.insert(curr_grid.clone());

    loop {
        curr_grid = step(&curr_grid.clone());
        if visited.contains(&curr_grid.clone()) {
            return curr_grid;
        }
        visited.insert(curr_grid.clone());
    }
}

fn calc_rating(input: &Grid) -> usize {
    let mut ctr: u32 = 0;
    let mut tot = 0;
    for y in 0..5 {
        for x in 0..5 {
            tot += if input[y][x] { 2_usize.pow(ctr) } else { 0 };
            ctr += 1;
        }
    }
    return tot;
}

fn count_indices(input: &Grid, indices: &[usize]) -> usize {
    let mut sum = 0;
    for idx in indices {
        let x = (idx - 1) % 5;
        let y = (idx - 1) / 5;
        sum += input[y][x] as usize;
    }
    return sum;
}

fn step_multi(input: &MultiGrid, grid_idx: i32) -> Grid {
    let grid_curr = input.get(&grid_idx).unwrap();
    let grid_above = input.get(&(grid_idx + 1)).unwrap();
    let grid_below = input.get(&(grid_idx - 1)).unwrap();

    let mut output = grid_curr.clone();

    for y in 0..5 {
        for x in 0..5 {
            // Skip middle
            if y == 2 && x == 2 {
                continue;
            }

            let is_outer_left_edge = x == 0;
            let is_outer_right_edge = x == 4;
            let is_outer_top_edge = y == 0;
            let is_outer_bottom_edge = y == 4;

            let is_inner_left_edge = x == 3 && y == 2;
            let is_inner_right_edge = x == 1 && y == 2;
            let is_inner_top_edge = x == 2 && y == 3;
            let is_inner_bottom_edge = x == 2 && y == 1;

            let mut count_left = 0;
            let mut count_right = 0;
            let mut count_top = 0;
            let mut count_bottom = 0;

            if is_outer_left_edge {
                // 1 6 11 16 21
                count_left += count_indices(&grid_above, &[12]);
            } else if is_inner_left_edge {
                count_left += count_indices(&grid_below, &[5, 10, 15, 20, 25]);
            } else {
                count_left += grid_curr[y][x - 1] as usize;
            }

            if is_outer_right_edge {
                count_right += count_indices(&grid_above, &[14]);
            } else if is_inner_right_edge {
                count_right += count_indices(&grid_below, &[1, 6, 11, 16, 21]);
            } else {
                count_right += grid_curr[y][x + 1] as usize;
            }

            if is_outer_top_edge {
                count_top += count_indices(&grid_above, &[8]);
            } else if is_inner_top_edge {
                count_top += count_indices(&grid_below, &[21, 22, 23, 24, 25]);
            } else {
                count_top += grid_curr[y - 1][x] as usize;
            }

            if is_outer_bottom_edge {
                let grid_above = input.get(&(grid_idx + 1)).unwrap();
                count_bottom += count_indices(&grid_above, &[18]);
            } else if is_inner_bottom_edge {
                count_bottom += count_indices(&grid_below, &[1, 2, 3, 4, 5]);
            } else {
                count_bottom += grid_curr[y + 1][x] as usize;
            }

            let is_bug = grid_curr[y][x];
            let num_neigh_bugs =
                count_left as u8 + count_right as u8 + count_top as u8 + count_bottom as u8;

            if is_bug && num_neigh_bugs != 1 {
                output[y][x] = false;
            } else if !is_bug && (num_neigh_bugs == 1 || num_neigh_bugs == 2) {
                output[y][x] = true;
            }
        }
    }
    return output;
}

fn solve_part_one(input: String) -> usize {
    let grid = parse_grid(input);
    let repeat = first_repeating(&grid);

    return calc_rating(&repeat);
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    let mut line = String::new();
    for y in 0..5 {
        for x in 0..5 {
            if grid[y][x] {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        line.push('\n');
    }
    print!("{}", line);
}

fn solve_part_two(input: String) -> usize {
    let grid = parse_grid(input);
    let mut multigrid = MultiGrid::new();

    const NUM_MINS: usize = 200;
    const MAX_DEPTH: i32 = NUM_MINS as i32 + 2;

    for i in 0..(2 * MAX_DEPTH) {
        multigrid.insert(i - MAX_DEPTH, empty_grid());
    }
    multigrid.insert(0, grid.clone());

    for _min in 0..NUM_MINS {
        let mut multigrid_next = multigrid.clone();
        for i in 0..(2 * MAX_DEPTH - 2) {
            let idx = i - MAX_DEPTH + 1;
            let grid_new = step_multi(&multigrid, idx);
            multigrid_next.insert(idx, grid_new);
        }
        //print_grid(&multigrid.get(&0).unwrap());
        multigrid = multigrid_next;
    }

    let mut tot_bugs = 0;
    for (_idx, grid) in &multigrid {
        for y in 0..5 {
            for x in 0..5 {
                tot_bugs += grid[y][x] as usize;
            }
        }
    }

    return tot_bugs;
}
pub fn solve() {
    let input = common::read_file("2019/day24/input");
    println!("Part one: {}", solve_part_one(input.clone()));
    println!("Part two: {}", solve_part_two(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let start = parse_grid(
            [
                "....#", //
                "#..#.", //
                "#..##", //
                "..#..", //
                "#....",
            ]
            .join("\n"),
        );

        let minute_1 = parse_grid(
            [
                "#..#.", //
                "####.", //
                "###.#", //
                "##.##", //
                ".##..", //
            ]
            .join("\n"),
        );

        let output = step(&start);
        assert_eq!(output, minute_1);

        let minute_2 = parse_grid(
            [
                "#####", //
                "....#", //
                "....#", //
                "...#.", //
                "#.###", //
            ]
            .join("\n"),
        );

        let output = step(&minute_1);
        assert_eq!(output, minute_2);

        let minute_3 = parse_grid(
            [
                "#....", //
                "####.", //
                "...##", //
                "#.##.", //
                ".##.#", //
            ]
            .join("\n"),
        );

        let output = step(&minute_2);
        assert_eq!(output, minute_3);

        let minute_4 = parse_grid(
            [
                "####.", //
                "....#", //
                "##..#", //
                ".....", //
                "##...", //
            ]
            .join("\n"),
        );

        let output = step(&minute_3);
        assert_eq!(output, minute_4);

        let first_repeat = parse_grid(
            [
                ".....", //
                ".....", //
                ".....", //
                "#....", //
                ".#...", //
            ]
            .join("\n"),
        );

        let output = first_repeating(&start);
        assert_eq!(output, first_repeat);
    }
}
