extern crate common;

fn solve_recursive(input: &Vec<char>, idx_start: usize) -> (usize, usize) {
    let mut lhs = 0;
    let mut last_operator = '#';

    let mut idx = idx_start;
    while idx < input.len() {
        let c = input[idx];
        let c_number = c.to_digit(10);
        if c == '(' {
            let (value, idx_new) = solve_recursive(input, idx + 1);
            lhs = match last_operator {
                '#' => value,
                '+' => lhs + value,
                '*' => lhs * value,
                _ => panic!("invalid operator"),
            };
            idx = idx_new - 1;
        } else if c == ')' {
            return (lhs, idx + 1);
        } else if c_number.is_some() {
            let value = c_number.unwrap() as usize;
            lhs = match last_operator {
                '#' => value,
                '+' => lhs + value,
                '*' => lhs * value,
                _ => panic!("invalid operator"),
            };
        } else if c == '*' || c == '+' {
            last_operator = c;
        } else {
            panic!("Invalid char {}", c);
        }

        idx += 1;
    }
    return (lhs, 0);
}

fn part_one(input: &String) -> usize {
    return input
        .replace(" ", "")
        .lines()
        .map(|line| solve_recursive(&String::from(line).chars().collect(), 0).0)
        .sum();
}

fn part_two(input: &String) -> usize {
    return input
        .lines()
        .map(|line| {
            solve_recursive(
                &format!(
                    "({})",
                    line.replace("(", "((")
                        .replace(")", "))")
                        .replace(" * ", ") * (")
                        .replace(" ", "")
                )
                .chars()
                .collect(),
                0,
            )
            .0
        })
        .sum();
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input_a = String::from("2 * 3 + (4 * 5)");
        let input_b = String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        let input_c = String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        let input_d = String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

        assert_eq!(part_one(&input_a), 26);
        assert_eq!(part_one(&input_b), 437);
        assert_eq!(part_one(&input_c), 12240);
        assert_eq!(part_one(&input_d), 13632);
    }

    #[test]
    fn test_samples_p2() {
        let input_a = String::from("1 + (2 * 3) + (4 * (5 + 6))");
        let input_b = String::from("2 * 3 + (4 * 5)");
        let input_c = String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        let input_d = String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        let input_e = String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

        assert_eq!(part_two(&input_a), 51);
        assert_eq!(part_two(&input_b), 46);
        assert_eq!(part_two(&input_c), 1445);
        assert_eq!(part_two(&input_d), 669060);
        assert_eq!(part_two(&input_e), 23340);
    }
}
