extern crate common;

use std::collections::HashSet;

fn solve_both(input: &String) -> (usize, usize) {
    let heights: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let x_len = heights[0].len();
    let y_len = heights.len();
    let mut visible = HashSet::new();
    let mut best_view = 0;

    for y in 0..y_len {
        for x in 0..x_len {
            let height = heights[y][x];
            let mut sum = 1;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let mut blocked = false;
                let mut curr_value = 0;
                let (mut curr_x, mut curr_y) = (x as i32 + dx, y as i32 + dy);
                while curr_x >= 0 && curr_y >= 0 && curr_x < x_len as i32 && curr_y < y_len as i32 {
                    curr_value += 1;
                    if heights[curr_y as usize][curr_x as usize] >= height {
                        blocked = true;
                        break;
                    }
                    curr_x += dx;
                    curr_y += dy;
                }
                sum = sum * curr_value;
                if !blocked {
                    visible.insert((x, y));
                }
            }
            best_view = std::cmp::max(best_view, sum);
        }
    }

    return (visible.len(), best_view);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let (p0, p1) = solve_both(&input);
    println!("Part one: {}", p0);
    println!("Part two: {}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "30373
25512
65332
33549
35390"
            .to_string();
        assert_eq!(solve_both(&input).0, 21);
        assert_eq!(solve_both(&input).1, 8);
    }
}
