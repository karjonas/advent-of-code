extern crate common;
use std::mem;
const INPUT_PATH: &str = "2015/day18/input";

fn parse_input(input: &str, part_two: bool) -> Vec<Vec<char>> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut ret = Vec::new();
    for line in lines {
        let mut line_chars = line.chars().collect::<Vec<char>>();
        ret.push(line_chars);
    }

    if part_two {
        let width = ret[0].len();
        let height = ret.len();

        ret[0][0] = '#';
        ret[height - 1][0] = '#';
        ret[height - 1][width - 1] = '#';
        ret[0][width - 1] = '#';
    }

    return ret;
}

fn next_object(pos: (usize, usize), grid: &Vec<Vec<char>>, part_two: bool) -> char {
    let c = grid[pos.1][pos.0];
    assert!(c == '#' || c == '.');
    let x = pos.0 as i64;
    let y = pos.1 as i64;
    let width = grid[0].len() as i64;
    let height = grid.len() as i64;

    if part_two {
        let corners = [
            (0, 0),
            (height - 1, 0),
            (height - 1, width - 1),
            (0, width - 1),
        ];
        if corners.contains(&(y, x)) {
            return '#';
        }
    }

    let mut neighs = Vec::new();
    neighs.reserve(8);

    let possibles = [
        (x, y - 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
    ];

    for (x, y) in possibles.iter() {
        if *x < width && *x >= 0 && *y < height && *y >= 0 {
            neighs.push(grid[*y as usize][*x as usize]);
        }
    }

    let num_neighs_on = neighs
        .iter()
        .fold(0, |sum, v| sum + if *v == '#' { 1 } else { 0 });

    if c == '#' {
        return if num_neighs_on == 2 || num_neighs_on == 3 {
            '#'
        } else {
            '.'
        };
    } else {
        return if num_neighs_on == 3 { '#' } else { '.' };
    }
}

fn run(grid: &Vec<Vec<char>>, grid_next: &mut Vec<Vec<char>>, part_two: bool) {
    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            grid_next[y][x] = next_object((x, y), &grid, part_two);
        }
    }
}

fn calc_value(grid: &Vec<Vec<char>>) -> usize {
    return grid.iter().fold(0, |sum, v| {
        sum + {
            v.iter()
                .fold(0, |sum1, v1| sum1 + if *v1 == '#' { 1 } else { 0 })
        }
    });
}

fn solve_internal(input: &str, num_iters: usize, part_two: bool) -> usize {
    let grid = parse_input(input, part_two);

    let mut grid_next = grid.clone();
    let mut grid_curr = grid.clone();

    for _i in 0..num_iters {
        mem::swap(&mut grid_curr, &mut grid_next);
        run(&grid_curr, &mut grid_next, part_two);
    }

    return calc_value(&grid_next);
}

pub fn solve() {
    println!(
        "Part one: {}",
        solve_internal(common::read_file(INPUT_PATH).as_str(), 100, false)
    );
    println!(
        "Part two: {}",
        solve_internal(common::read_file(INPUT_PATH).as_str(), 100, true)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [".#.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.."].join("\n");
        assert_eq!(solve_internal(input.as_str(), 4, false), 4);
        assert_eq!(solve_internal(input.as_str(), 5, true), 17);
    }
}
