extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Line {
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
}

fn parse(input: &String) -> Vec<Line> {
    let mut result = Vec::new();
    for line in input.lines() {
        let (x0, y0, x1, y1) = scan_fmt!(line, "{d},{d} -> {d},{d}", i64, i64, i64, i64).unwrap();
        result.push(Line {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
        });
    }
    return result;
}

fn solve_internal(input: &String, include_diagonal: bool) -> usize {
    let lines = parse(input);
    let mut points = HashMap::new();

    for line in lines {
        let is_diagonal = line.x0 != line.x1 && line.y0 != line.y1;
        if !include_diagonal && is_diagonal {
            continue;
        }

        let num_steps = std::cmp::max((line.x1 - line.x0).abs(), (line.y1 - line.y0).abs()) + 1;
        let mut dir_x = if line.x1 > line.x0 { 1 } else { -1 };
        let mut dir_y = if line.y1 > line.y0 { 1 } else { -1 };
        if !is_diagonal {
            dir_x = if line.x0 == line.x1 { 0 } else { dir_x };
            dir_y = if line.y0 == line.y1 { 0 } else { dir_y };
        }

        for i in 0..num_steps {
            let pos = (line.x0 + dir_x * i, line.y0 + dir_y * i);
            let entry = points.entry(pos).or_insert(0);
            *entry += 1;
        }
    }

    let mut ctr = 0;
    for (_k, &v) in &points {
        if v >= 2 {
            ctr += 1;
        }
    }

    return ctr;
}

fn solve_internal_p1(input: &String) -> usize {
    return solve_internal(input, false);
}

fn solve_internal_p2(input: &String) -> usize {
    return solve_internal(input, true);
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
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(solve_internal_p1(&String::from(input)), 5);
        assert_eq!(solve_internal_p2(&String::from(input)), 12);
    }
}
