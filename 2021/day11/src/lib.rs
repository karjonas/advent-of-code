extern crate common;

use std::collections::HashSet;

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().map(|v| v as u8 - '0' as u8).collect());
    }
    return grid;
}

fn do_step(mut grid: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut flashed = HashSet::new();

    for y in 0..rows {
        for x in 0..cols {
            grid[y][x] += 1;
        }
    }
    let mut grid_next = grid.clone();

    let mut any_change = true;
    while any_change {
        any_change = false;
        for y in 0..rows {
            for x in 0..cols {
                if grid[y][x] > 9 && !flashed.contains(&(x, y)) {
                    any_change = true;
                    flashed.insert((x, y));
                    for (dx, dy) in [
                        (0, -1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                        (0, 1),
                        (-1, 1),
                        (-1, 0),
                        (-1, -1),
                    ] {
                        let y_adj = y as i32 + dy;
                        let x_adj = x as i32 + dx;

                        if y_adj >= rows as i32 || y_adj < 0 || x_adj >= cols as i32 || x_adj < 0 {
                            continue;
                        }

                        grid_next[y_adj as usize][x_adj as usize] += 1;
                    }
                }
            }
        }

        grid = grid_next.clone();
    }

    for (x, y) in &flashed {
        grid[*y][*x] = 0;
    }

    return (grid, flashed.len());
}

fn solve_internal_p1(input: &String) -> usize {
    let mut grid = parse_input(input);

    let mut flashes_sum = 0;
    for _step in 0..100 {
        let (grid_next, flashes) = do_step(grid.clone());
        flashes_sum += flashes;

        grid = grid_next;
    }

    return flashes_sum;
}

fn solve_internal_p2(input: &String) -> usize {
    let mut grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut ctr = 0;
    loop {
        ctr += 1;
        let (grid_next, flashes) = do_step(grid.clone());
        if flashes == rows * cols {
            break;
        }
        grid = grid_next;
    }

    return ctr;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(solve_internal_p1(&String::from(input)), 1656);
        assert_eq!(solve_internal_p2(&String::from(input)), 195);
    }
}
