extern crate common;

fn solve_internal(input: &String, part_one: bool) -> usize {
    let mut board = common::filled_vector(1000, common::filled_vector(1000, 0 as i64));

    for line in input.lines() {
        let words = line
            .replace(",", " ")
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        if words[0] == "toggle" {
            let pos_a = (
                common::string_to_i64(words[1].as_str()) as usize,
                common::string_to_i64(words[2].as_str()) as usize,
            );
            let pos_b = (
                common::string_to_i64(words[4].as_str()) as usize + 1,
                common::string_to_i64(words[5].as_str()) as usize + 1,
            );

            for x in pos_a.0..pos_b.0 {
                for y in pos_a.1..pos_b.1 {
                    if part_one {
                        board[y][x] = if board[y][x] == 1 { 0 } else { 1 };
                    } else {
                        board[y][x] += 2;
                    }
                }
            }
        } else if words[0] == "turn" {
            let turn_on = words[1] == "on";
            let pos_a = (
                common::string_to_i64(words[2].as_str()) as usize,
                common::string_to_i64(words[3].as_str()) as usize,
            );
            let pos_b = (
                common::string_to_i64(words[5].as_str()) as usize + 1,
                common::string_to_i64(words[6].as_str()) as usize + 1,
            );
            for x in pos_a.0..pos_b.0 {
                for y in pos_a.1..pos_b.1 {
                    if part_one {
                        board[y][x] = if turn_on { 1 } else { 0 };
                    } else {
                        board[y][x] += if turn_on { 1 } else { -1 };
                        board[y][x] = std::cmp::max(0, board[y][x]);
                    }
                }
            }
        } else {
            assert!(false);
        }
    }

    let mut ctr = 0;
    for line in board.iter() {
        for light in line.iter() {
            ctr += *light as usize;
        }
    }

    return ctr;
}

pub fn solve(filepath: &str) {
    println!(
        "Part one: {}",
        solve_internal(
            &std::fs::read_to_string(filepath)
                .unwrap()
                .trim()
                .to_string(),
            true
        )
    );
    println!(
        "Part two: {}",
        solve_internal(
            &std::fs::read_to_string(filepath)
                .unwrap()
                .trim()
                .to_string(),
            false
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(
            solve_internal(&"turn on 0,0 through 999,999".to_string(), true),
            1000000
        );

        assert_eq!(
            solve_internal(&"toggle 0,0 through 999,0".to_string(), true),
            1000
        );
    }

    #[test]
    fn test_samples_part_two() {}
}
