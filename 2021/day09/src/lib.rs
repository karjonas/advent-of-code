extern crate common;

use std::collections::HashSet;

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().map(|v| v as u8 - '0' as u8).collect());
    }
    return grid;
}

fn solve_internal_p1(input: &String) -> usize {
    let grid = parse_input(input);
    let mut low_points = Vec::<u8>::new();

    let cols = grid[0].len() as i32;
    let rows = grid.len() as i32;

    for y in 0..rows {
        for x in 0..cols {
            let number = grid[y as usize][x as usize];
            let mut neighs = Vec::new();

            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let y_adj = y + dy;
                let x_adj = x + dx;

                if y_adj >= rows || y_adj < 0 || x_adj >= cols || x_adj < 0 {
                    continue;
                }
                neighs.push(grid[y_adj as usize][x_adj as usize]);
            }

            if neighs.iter().find(|&v| number >= *v) == None {
                low_points.push(number);
            }
        }
    }

    return low_points.iter().map(|&v| v as usize + 1).sum();
}

fn flood(pos: (i32, i32), grid: &Vec<Vec<u8>>, visited: &mut HashSet<(i32, i32)>) -> usize {
    if grid[pos.1 as usize][pos.0 as usize] == 9 || visited.contains(&pos) {
        return 0;
    }

    visited.insert(pos);
    let mut sum = 1;

    let cols = grid[0].len() as i32;
    let rows = grid.len() as i32;
    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let y_adj = pos.1 + dy;
        let x_adj = pos.0 + dx;

        if y_adj >= rows || y_adj < 0 || x_adj >= cols || x_adj < 0 {
            continue;
        }
        sum += flood((x_adj, y_adj), grid, visited);
    }

    return sum;
}

fn solve_internal_p2(input: &String) -> usize {
    let grid = parse_input(input);

    let cols = grid[0].len() as i32;
    let rows = grid.len() as i32;

    let mut visited = HashSet::new();

    let mut sums = Vec::new();
    for y in 0..rows {
        for x in 0..cols {
            let sum = flood((x, y), &grid, &mut visited);
            sums.push(sum);
        }
    }

    sums.sort_by(|a, b| b.cmp(a));
    return sums[0] * sums[1] * sums[2];
}

pub fn solve() {
    let input = common::read_file("2021/day09/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(solve_internal_p1(&String::from(input)), 15);
        assert_eq!(solve_internal_p2(&String::from(input)), 1134);
    }
}
