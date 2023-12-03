extern crate common;
extern crate regex;

use regex::Regex;

const SOURCE_POS: usize = 500;

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let lines = input.lines().collect::<Vec<_>>();

    let re = Regex::new(r"(\w)=(\d+), (\w)=(\d+)..(\d+)").unwrap();

    let mut ranges: Vec<(usize, usize, usize, usize)> = Vec::new();

    for line in lines {
        let cap = re.captures(line).unwrap();
        let axis_a = &cap[1].to_string();
        let axis_b = &cap[3].to_string();
        let value_a = common::string_to_i64(&cap[2]) as usize;
        let value_b = (
            common::string_to_i64(&cap[4]) as usize,
            common::string_to_i64(&cap[5]) as usize,
        );

        if axis_a == "x" {
            ranges.push((value_a, value_a + 1, value_b.0, value_b.1 + 1));
        } else if axis_b == "x" {
            ranges.push((value_b.0, value_b.1 + 1, value_a, value_a + 1));
        }
    }

    let mut max_x = SOURCE_POS;
    let mut max_y = 0;
    let mut min_x = std::usize::MAX;

    for range in &ranges {
        min_x = std::cmp::min(min_x, range.0);
        max_x = std::cmp::max(max_x, range.1);
        max_y = std::cmp::max(max_y, range.3);
    }

    let mut out = common::filled_vector(max_y, common::filled_vector(max_x + 1, '.'));

    for range in &ranges {
        for x in range.0..range.1 {
            for y in range.2..range.3 {
                out[y][x] = '#';
            }
        }
    }

    out[0][SOURCE_POS] = '+';

    for line in &out {
        let mut s = String::new();
        for i in min_x..line.len() {
            s.push(line[i]);
        }
    }

    return out;
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let mut min_x = std::usize::MAX;
    for line in grid {
        for i in 0..line.len() {
            if line[i] != '.' {
                min_x = std::cmp::min(min_x, i);
                break;
            }
        }
    }

    for line in grid {
        let mut s = String::new();
        for i in min_x..line.len() {
            s.push(line[i]);
        }
        println!("{}", s);
    }
}

fn fill(grid: &mut Vec<Vec<char>>, x: usize, y: usize, dir: i64) -> usize {
    if grid[y][x] == '.' {
        grid[y][x] = '|';
    }
    if y == grid.len() - 1 {
        return 0;
    }
    if grid[y][x] == '#' {
        return x;
    }
    if grid[y + 1][x] == '.' {
        fill(grid, x, y + 1, 0);
    }
    if grid[y + 1][x] == '~' || grid[y + 1][x] == '#' {
        if dir != 0 {
            return fill(grid, (x as i64 + dir) as usize, y, dir);
        } else {
            let left = fill(grid, x - 1, y, -1);
            let right = fill(grid, x + 1, y, 1);
            if grid[y][left] == '#' && grid[y][right] == '#' {
                for i in (left + 1)..right {
                    grid[y][i] = '~';
                }
            }
        }
    }

    return x;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let mut grid = parse_input(&input);
    let height = grid.len();
    let width = grid[0].len();

    let mut smallest_y = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '#' {
                smallest_y = y;
                break;
            }
        }
        if smallest_y > 0 {
            break;
        }
    }

    fill(&mut grid, SOURCE_POS, 0, 0);

    let mut num_flow = 0;
    let mut num_still = 0;
    for y in smallest_y..height {
        for x in 0..width {
            if grid[y][x] == '~' {
                num_still += 1;
            } else if grid[y][x] == '|' {
                num_flow += 1
            }
        }
    }

    if false {
        print_grid(&grid);
    }

    println!("Part one: {}", num_flow + num_still);
    println!("Part two: {}", num_still)
}
