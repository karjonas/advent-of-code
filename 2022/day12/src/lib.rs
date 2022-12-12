use std::collections::HashSet;
use std::collections::VecDeque;

extern crate common;

fn solve_both(input: &String, part_two: bool) -> usize {
    let mut map: Vec<Vec<usize>> = input
        .lines()
        .map(|s| s.chars().map(|c| c as usize).collect())
        .collect();
    let width = map[0].len();
    let height = map.len();
    const START: usize = 'S' as usize;
    const END: usize = 'E' as usize;
    const A_LOWER: usize = 'a' as usize;
    const Z_LOWER: usize = 'z' as usize;

    let mut stack = VecDeque::new();
    let mut end_pos = (0, 0);

    for r in 0..height {
        for c in 0..width {
            if map[r][c] == START || (part_two && map[r][c] == A_LOWER) {
                stack.push_back((0, (c, r)));
                map[r][c] = A_LOWER;
            } else if map[r][c] == END {
                end_pos = (c, r);
                map[r][c] = Z_LOWER;
            }
        }
    }

    let mut visited = HashSet::new();

    while !stack.is_empty() {
        let (steps, (x, y)) = stack.pop_front().unwrap();
        if (x, y) == end_pos {
            return steps;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        let curr = map[y][x];

        if y + 1 < height && (map[y + 1][x] <= curr + 1) {
            stack.push_back((steps + 1, (x, y + 1)));
        }
        if y > 0 && (map[y - 1][x] <= curr + 1) {
            stack.push_back((steps + 1, (x, y - 1)));
        }
        if x + 1 < width && (map[y][x + 1] <= curr + 1) {
            stack.push_back((steps + 1, (x + 1, y)));
        }
        if x > 0 && (map[y][x - 1] <= curr + 1) {
            stack.push_back((steps + 1, (x - 1, y)));
        }
    }
    return 0;
}

fn part_one(input: &String) -> usize {
    solve_both(input, false)
}

fn part_two(input: &String) -> usize {
    solve_both(input, true)
}

pub fn solve() {
    let input = common::read_file("2022/day12/input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            .to_string();
        assert_eq!(part_one(&input), 31);
        assert_eq!(part_two(&input), 29);
    }
}
