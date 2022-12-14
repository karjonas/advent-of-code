use std::collections::HashSet;

extern crate common;

fn parse_input(input: &String) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    for line in input.lines() {
        let numbers: Vec<usize> = line
            .replace(" -> ", " ")
            .replace(",", " ")
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        for pair_i in 0..(numbers.len() - 1) / 2 {
            let i = pair_i * 2;
            let p0 = (
                std::cmp::min(numbers[i], numbers[i + 2]),
                std::cmp::min(numbers[i + 1], numbers[i + 3]),
            );
            let p1 = (
                std::cmp::max(numbers[i], numbers[i + 2]),
                std::cmp::max(numbers[i + 1], numbers[i + 3]),
            );
            for x in p0.0..p1.0 + 1 {
                for y in p0.1..p1.1 + 1 {
                    result.insert((x, y));
                }
            }
        }
    }

    return result;
}

fn solve_internal(input: &String, part_one: bool) -> usize {
    let walls = parse_input(input);
    let max_y = walls.iter().fold(0, |v, (_, y)| std::cmp::max(v, *y));
    let max_x = walls.iter().fold(0, |v, (x, _)| std::cmp::max(v, *x));

    let mut is_wall = vec![vec![false; max_y as usize + 3]; max_x as usize + max_y as usize];
    for (x, y) in walls {
        is_wall[x][y] = true;
    }

    for x in 0..max_x + max_y {
        is_wall[x][max_y + 2] = true;
    }

    for i in 0..std::usize::MAX {
        let mut pos = (500, 0);
        loop {
            if (part_one && pos.1 > max_y) || is_wall[pos.0][pos.1] {
                return i;
            }

            let below = (pos.0, pos.1 + 1);
            let left = (pos.0 - 1, pos.1 + 1);
            let right = (pos.0 + 1, pos.1 + 1);

            if !is_wall[below.0][below.1] {
                pos = below;
                continue;
            } else if !is_wall[left.0][left.1] {
                pos = left;
                continue;
            } else if !is_wall[right.0][right.1] {
                pos = right;
                continue;
            } else {
                is_wall[pos.0][pos.1] = true;
                break;
            }
        }
    }

    return 0;
}

fn part_one(input: &String) -> usize {
    return solve_internal(input, true);
}

fn part_two(input: &String) -> usize {
    return solve_internal(input, false);
}

pub fn solve() {
    let input = common::read_file("2022/day14/input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            .to_string();
        assert_eq!(part_one(&input), 24);
        assert_eq!(part_two(&input), 93);
    }
}
