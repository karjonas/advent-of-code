extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Instructions {
    dots: HashSet<(usize, usize)>,
    folds: Vec<(char, usize)>,
}

fn parse_input(input: &String) -> Instructions {
    let mut result = Instructions {
        dots: HashSet::new(),
        folds: Vec::new(),
    };

    let mut parsing_dots = true;
    for line in input.lines() {
        if line == "" {
            parsing_dots = false;
            continue;
        }

        if parsing_dots {
            let (x, y) = scan_fmt!(line, "{d},{d}", usize, usize).unwrap();
            result.dots.insert((x, y));
        } else {
            let (c, d) = scan_fmt!(line, "fold along {}={d}", char, usize).unwrap();
            result.folds.push((c, d));
        }
    }

    return result;
}

fn draw_map(map: &HashSet<(usize, usize)>) -> String {
    let mut min = (0, 0);
    let mut max = (0, 0);

    for k in map {
        min = (std::cmp::min(min.0, k.0), std::cmp::min(min.1, k.1));
        max = (std::cmp::max(max.0, k.0), std::cmp::max(max.1, k.1));
    }

    let w = 1 + max.0 - min.0;
    let h = 1 + max.1 - min.1;

    let mut s = String::new();

    for y in 0..h {
        for x in 0..w {
            let color = map.contains(&(min.0 + x, min.1 + y));
            s.push(if !color { ' ' } else { '#' });
        }
        s.push('\n')
    }

    return s;
}

fn solve_internal(input: &String, part_one: bool) -> HashSet<(usize, usize)> {
    let parsed = parse_input(input);
    let mut dots = parsed.dots;

    for (c, fold) in parsed.folds {
        let mut dots_next = HashSet::new();
        if c == 'y' {
            for &(x, y) in &dots {
                if y > fold {
                    let y_adj = 2 * fold - y;
                    dots_next.insert((x, y_adj));
                } else {
                    dots_next.insert((x, y));
                }
            }
        }
        if c == 'x' {
            for &(x, y) in &dots {
                if x > fold {
                    let x_adj = 2 * fold - x;
                    dots_next.insert((x_adj, y));
                } else {
                    dots_next.insert((x, y));
                }
            }
        }
        dots = dots_next;
        if part_one {
            break;
        }
    }

    return dots;
}

fn solve_internal_p1(input: &String) -> usize {
    let solved = solve_internal(input, true);
    return solved.len();
}

fn solve_internal_p2(input: &String) -> String {
    let solved = solve_internal(input, false);
    return draw_map(&solved);
}

pub fn solve() {
    let input = common::read_file("2021/day13/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two:\n{}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        assert_eq!(solve_internal_p1(&String::from(input)), 17);
    }
}
