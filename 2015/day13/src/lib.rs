extern crate common;

use std::collections::HashMap;

fn solve_internal(input: String, part_two: bool) -> usize {
    let mut person_id = HashMap::new();

    let mut max_idx = -1;
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let idx = person_id
            .entry(words[0])
            .or_insert((max_idx + 1) as usize)
            .clone();
        max_idx = std::cmp::max(max_idx, idx as i64);
    }

    let num_persons = max_idx as usize + 1 + if part_two { 1 } else { 0 };

    let mut grid = common::filled_vector(num_persons, common::filled_vector(num_persons, 0));

    for line in input.lines() {
        let line_clean = common::strip_characters(line, ".").clone();
        let words = line_clean
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let idx_a = *person_id.get(&words[0].as_str()).unwrap();
        let gain = if words[2] == "gain" { 1 } else { -1 };
        let happiness = gain * common::string_to_i64(words[3].as_str());
        let idx_b = *person_id.get(&words[10].as_str()).unwrap();

        grid[idx_a][idx_b] = happiness;
    }

    if part_two {
        for i in 0..num_persons {
            grid[num_persons - 1][i] = 0;
            grid[i][num_persons - 1] = 0;
        }
    }

    let perms = common::permute(&Vec::new(), &(0..num_persons).collect::<Vec<usize>>());

    let mut best_score = 0;

    for perm in &perms {
        let mut sum = 0;

        for i in 0..num_persons {
            let idx = perm[i];
            let left_idx = perm[(num_persons + i - 1) % num_persons];
            let right_idx = perm[(i + 1) % num_persons];

            let val = grid[idx][left_idx] + grid[idx][right_idx];
            sum += val;
        }

        best_score = std::cmp::max(best_score, sum);
    }

    return best_score as usize;
}

pub fn solve() {
    println!(
        "Part one: {}",
        solve_internal(common::read_file("2015/day13/input"), false)
    );
    println!(
        "Part two: {}",
        solve_internal(common::read_file("2015/day13/input"), true)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input = [
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol.",
        ]
        .join("\n");

        assert_eq!(solve_internal(input, false), 330);
    }
}
