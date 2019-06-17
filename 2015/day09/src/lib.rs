extern crate common;

use std::collections::HashMap;

fn distances(input: String) -> Vec<Vec<usize>> {
    let mut places = HashMap::new();
    let mut max_idx = -1 as i64;

    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let from = words[0];
        let to = words[2];

        let idx_a = places.entry(from).or_insert((max_idx + 1) as usize).clone();
        max_idx = std::cmp::max(max_idx, idx_a as i64);
        let idx_b = places.entry(to).or_insert((max_idx + 1) as usize).clone();
        max_idx = std::cmp::max(max_idx, idx_b as i64);
    }

    let mut distances = common::filled_vector(
        max_idx as usize + 1,
        common::filled_vector(max_idx as usize + 1, 0),
    );

    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let from = words[0];
        let to = words[2];

        let from_idx = *places.get(&from).unwrap();
        let to_idx = *places.get(&to).unwrap();

        distances[from_idx][to_idx] = common::string_to_i64(words[4]) as usize;
        distances[to_idx][from_idx] = common::string_to_i64(words[4]) as usize;
    }

    return distances;
}

fn solve_internal(input: String, part_one: bool) -> usize {
    let distances = distances(input);
    let num_places = distances.len();
    let rest = (0..num_places).collect::<Vec<usize>>();
    let paths = common::permute(&Vec::new(), &rest);

    let mut best_dist = if part_one { std::usize::MAX } else { 0 };

    for path in paths {
        let mut dist_accum = 0;
        let mut invalid = false;
        for i in 0..path.len() - 1 {
            let idx_from = path[i];
            let idx_to = path[i + 1];
            let dist = distances[idx_from][idx_to];

            dist_accum += dist;
            if dist == 0 {
                invalid = true;
                break;
            }
        }

        if !invalid {
            best_dist = if part_one {
                std::cmp::min(best_dist, dist_accum)
            } else {
                std::cmp::max(best_dist, dist_accum)
            };
        }
    }

    return best_dist;
}

pub fn solve() {
    println!(
        "Part one: {}",
        solve_internal(common::read_file("2015/day09/input"), true)
    );

    println!(
        "Part two: {}",
        solve_internal(common::read_file("2015/day09/input"), false)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

        assert_eq!(solve_internal(input.to_string(), true), 605);
    }
}
