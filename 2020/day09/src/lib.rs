extern crate common;

fn parse_input(input: &String) -> Vec<usize> {
    return input.lines().map(|v| v.parse::<usize>().unwrap()).collect();
}

fn solve_numbers(numbers: &Vec<usize>, preamble: usize) -> usize {
    let numbers_len = numbers.len();
    for i in preamble..numbers_len {
        let mut success = false;
        for j in i - preamble..i {
            for k in j + 1..i {
                if numbers[j] + numbers[k] == numbers[i] {
                    success = true;
                    break;
                }
            }
            if success {
                break;
            }
        }

        if !success {
            return numbers[i];
        }
    }

    panic!("No solution found");
}

fn solve_numbers_p2(numbers: &Vec<usize>, preamble: usize) -> usize {
    let number_found = solve_numbers(numbers, preamble);

    let numbers_len = numbers.len();
    for i in 0..numbers_len {
        if numbers[i] == number_found {
            continue;
        }

        let mut sum = numbers[i];
        for j in i + 1..numbers_len {
            sum += numbers[j];
            if sum == number_found {
                return numbers[i..j].iter().min().unwrap() + numbers[i..j].iter().max().unwrap();
            }
            if sum > number_found {
                break;
            }
        }
    }

    panic!("No solution found");
}

fn part_one(numbers: &Vec<usize>) -> usize {
    return solve_numbers(numbers, 25) as usize;
}

fn part_two(numbers: &Vec<usize>) -> usize {
    return solve_numbers_p2(numbers, 25) as usize;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let numbers = parse_input(&input);
    println!("Part one: {}", part_one(&numbers));
    println!("Part two: {}", part_two(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
        .join("\n");
        assert_eq!(solve_numbers(&parse_input(&input), 5), 127);
        assert_eq!(solve_numbers_p2(&parse_input(&input), 5), 62);
    }
}
