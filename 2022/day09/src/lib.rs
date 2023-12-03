extern crate common;

use std::collections::HashSet;

fn simulate(input: &String, rope_length: usize) -> usize {
    let mut r: Vec<(i32, i32)> = vec![(0, 0); rope_length];
    let last_idx = rope_length - 1;

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for line in input.lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        let steps: i32 = words[1].parse().unwrap();

        for _ in 0..steps {
            match words[0] {
                "U" => {
                    r[0] = (r[0].0, r[0].1 + 1);
                }
                "D" => {
                    r[0] = (r[0].0, r[0].1 - 1);
                }
                "L" => {
                    r[0] = (r[0].0 - 1, r[0].1);
                }
                "R" => {
                    r[0] = (r[0].0 + 1, r[0].1);
                }
                _ => assert!(false),
            }

            for i in 1..r.len() {
                let h = r[i - 1];
                let t = r[i];

                let dist = std::cmp::max((h.0 - t.0).abs(), (h.1 - t.1).abs());
                if dist < 2 {
                    continue;
                }

                let mut t_next = t;
                let mut dist_next = std::f32::MAX;

                for (dx, dy) in [
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                ] {
                    let t_try = (t.0 + dx, t.1 + dy);
                    let dist_try = (((h.0 - t_try.0).abs() + (h.1 - t_try.1).abs()) as f32).sqrt();
                    if dist_try < dist_next {
                        t_next = t_try;
                        dist_next = dist_try;
                    }
                }

                if i == last_idx {
                    visited.insert(t_next);
                }
                r[i] = t_next;
            }
        }
    }

    return visited.len();
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    println!("Part one: {}", simulate(&input, 2));
    println!("Part two: {}", simulate(&input, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .to_string();
        assert_eq!(simulate(&input, 2), 13);
    }

    #[test]
    fn test_samples_p2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .to_string();
        assert_eq!(simulate(&input, 10), 36);
    }
}
