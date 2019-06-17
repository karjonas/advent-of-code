extern crate common;

use std::collections::HashSet;

fn solve_internal(s: &str, with_robot: bool) -> usize {
    let mut visited = HashSet::new();
    let mut pos_santa = (0 as i64, 0 as i64);
    let mut pos_robot = pos_santa.clone();
    visited.insert(pos_santa);
    let mut santa_move = true;

    for c in s.chars() {
        let mut pos: &mut (i64, i64) = if with_robot && !santa_move {
            &mut pos_robot
        } else {
            &mut pos_santa
        };

        pos.0 += if c == '<' { -1 } else { 0 };
        pos.0 += if c == '>' { 1 } else { 0 };
        pos.1 += if c == '^' { -1 } else { 0 };
        pos.1 += if c == 'v' { 1 } else { 0 };

        visited.insert(pos.clone());

        santa_move = !santa_move;
    }

    return visited.len();
}

pub fn solve() {
    let input = common::read_file("2015/day03/input");
    println!("Part one: {}", solve_internal(input.as_str(), false));
    println!("Part two: {}", solve_internal(input.as_str(), true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(solve_internal(">", false), 2);
        assert_eq!(solve_internal("^>v<", false), 4);
        assert_eq!(solve_internal("^v^v^v^v^v", false), 2);
    }

    #[test]
    fn test_samples_part_two() {
        assert_eq!(solve_internal("^v", true), 3);
        assert_eq!(solve_internal("^>v<", true), 3);
        assert_eq!(solve_internal("^v^v^v^v^v", true), 11);
    }

}
